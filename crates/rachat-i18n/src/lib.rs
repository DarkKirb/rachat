//! Locwale handling code of rachat
use std::{
    borrow::Cow,
    collections::HashMap,
    str::FromStr,
    sync::{Arc, LazyLock},
};

use arc_swap::ArcSwap;
use eyre::{OptionExt, Result, eyre};
use fluent_bundle::{FluentResource, FluentValue};
use fluent_langneg::{
    LanguageIdentifier as LangnegIdentifier, NegotiationStrategy, convert_vec_str_to_langids_lossy,
    negotiate_languages,
};
use fluent_templates::{langid, static_loader};
use nonempty::NonEmpty;
use rachat_config::{Config, ConfigSourceExt};
use tokio::{select, sync::Notify};
use unic_langid_impl::LanguageIdentifier;

static_loader! {
    static LOCALES = {
        locales: "locales",
        fallback_language: "en-US",
        customise: |bundle| {
            // Since this will be called for each locale bundle and
            // `FluentResource`s need to be either `&'static` or behind an
            // `Arc` it's recommended you use lazily initialised
            // static variables.
            static CRATE_META_FTL: LazyLock<FluentResource> = LazyLock::new(|| {
                let ftl_string = String::from(
                    concat!("-crate-version = ", env!("CARGO_PKG_VERSION"), "\n-crate-license = ", env!("CARGO_PKG_LICENSE"), "\n-crate-repository = ", env!("CARGO_PKG_REPOSITORY"))
                );

                #[allow(clippy::unwrap_used)]
                FluentResource::try_new(ftl_string).unwrap()
            });

            #[allow(clippy::unwrap_used)]
            bundle.add_resource(&CRATE_META_FTL).unwrap();
        }
    };
}

/// Rachat localization helper
#[derive(Debug)]
pub struct Localizer {
    /// Selected languages
    langs: Arc<ArcSwap<NonEmpty<LanguageIdentifier>>>,
    /// Notifier when shutting down
    shutdown_notify: Arc<Notify>,
}

impl Localizer {
    /// Negotiates the languages with user language selection
    ///
    /// # Errors
    /// Returns an error if the default lanugage id failed to parse
    fn negotiate_languages(langs: &[LangnegIdentifier]) -> Result<NonEmpty<LanguageIdentifier>> {
        static AVAILABLE_LANGIDS: LazyLock<Vec<LangnegIdentifier>> =
            LazyLock::new(|| convert_vec_str_to_langids_lossy(["de-DE", "en-US", "nl-NL", "tok"]));
        let default = "en-US".parse().map_err(|e| eyre!("{e:?}"))?;
        let languages = negotiate_languages(
            langs,
            &AVAILABLE_LANGIDS,
            Some(&default),
            NegotiationStrategy::Filtering,
        );

        let mut has_english = false;
        let mut negotiated_langs = Vec::with_capacity(languages.len().max(1));

        for lang in languages {
            if lang.normalizing_eq("en-US") {
                has_english = true;
            }
            negotiated_langs.push(lang.to_string().parse()?);
        }

        if !has_english {
            negotiated_langs.push(langid!("en-US"));
        }

        NonEmpty::from_vec(negotiated_langs).ok_or_eyre("Language list should not be empty!")
    }

    /// Updates language list
    fn update_langs(
        cfg: &Config,
        tgt_langs: &Arc<ArcSwap<NonEmpty<LanguageIdentifier>>>,
    ) -> Result<()> {
        let mut langs: Vec<LangnegIdentifier> = cfg.get("i18n.langs")?.unwrap_or_default();
        langs.push(LangnegIdentifier::from_str("en-US").map_err(|e| eyre!("{e:?}"))?); // fallback language
        let langs = Self::negotiate_languages(&langs)?;
        tgt_langs.store(Arc::new(langs));
        Ok(())
    }
    /// Creates a new localizer from a config object
    ///
    /// # Errors
    ///
    /// This function returns an error if the language codes in i18n.langs are invalid
    pub fn new(cfg: &Config) -> Result<Arc<Self>> {
        let mut langs: Vec<LangnegIdentifier> = cfg.get("i18n.langs")?.unwrap_or_default();
        langs.push(LangnegIdentifier::from_str("en-US").map_err(|e| eyre!("{e:?}"))?); // fallback language

        let langs = Self::negotiate_languages(&langs)?;
        let watcher = cfg.watch_property("i18n.langs");
        let langs = Arc::new(ArcSwap::from_pointee(langs));
        let shutdown_notify = Arc::new(Notify::new());
        let shutdown_notify2 = Arc::clone(&shutdown_notify);
        let weak_langs = Arc::downgrade(&langs);
        let cfg2 = Arc::clone(cfg);

        #[allow(clippy::redundant_pub_crate)]
        tokio::spawn(async move {
            loop {
                select! {
                    () = watcher.notified() => {
                        if let Some(langs) = weak_langs.upgrade() {
                            if let Err(e) = Self::update_langs(&cfg2, &langs) {
                                let error = format!("{e:?}");
                                error!(failed_updating_languages, error = error);
                            }
                        } else {
                            break;
                        }
                    }
                    () = shutdown_notify2.notified() => break
                }
            }
        });

        let own_arc = Arc::new(Self {
            langs,
            shutdown_notify,
        });

        *ඞ::LOCALIZER.write() = Some(Arc::downgrade(&own_arc));

        Ok(own_arc)
    }

    /// Looks up a certain translation
    #[must_use]
    pub fn lookup(&self, text_id: &str) -> String {
        self.lookup_args_inner(text_id, None)
    }

    /// Looks up a certain translation and interpolates it with the given arguments
    #[must_use]
    pub fn lookup_args(
        &self,
        text_id: &str,
        args: &HashMap<Cow<'static, str>, FluentValue<'_>>,
    ) -> String {
        self.lookup_args_inner(text_id, Some(args))
    }

    /// Looks up a certain translation and interpolates it with the given arguments
    pub(crate) fn lookup_args_inner(
        &self,
        text_id: &str,
        args: Option<&HashMap<Cow<'static, str>, FluentValue<'_>>>,
    ) -> String {
        for lang in self.langs.load().iter() {
            if let Some(v) = LOCALES.lookup_no_default_fallback(lang, text_id, args) {
                return v;
            }
        }
        format!("[!!!UNKNOWN!!! text_id = {text_id}, args = {args:?}]")
    }
}

impl Drop for Localizer {
    fn drop(&mut self) {
        self.shutdown_notify.notify_one();
    }
}

/// This is an internal module used exclusively by macros.
///
/// This can change at any time. Do not use this in production code.
#[doc(hidden)]
pub mod ඞ {
    use std::{
        borrow::Cow,
        collections::HashMap,
        sync::{LazyLock, Weak},
    };

    use fluent_bundle::FluentValue;
    use parking_lot::RwLock;

    use crate::Localizer;

    pub(crate) static LOCALIZER: LazyLock<RwLock<Option<Weak<Localizer>>>> =
        LazyLock::new(|| RwLock::new(None));

    #[allow(clippy::implicit_hasher)]
    pub fn localize(
        text_id: &str,
        args: Option<&HashMap<Cow<'static, str>, FluentValue<'_>>>,
    ) -> String {
        if let Some(localizer) = &*LOCALIZER.read() {
            if let Some(localizer) = localizer.upgrade() {
                return localizer.lookup_args_inner(text_id, args);
            }
        }
        format!("ERROR: Localizer uninitialized! text_id = {text_id}, args = {args:?}")
    }
}

#[doc(hidden)]
pub use fluent_bundle;
#[doc(hidden)]
pub use tracing;

mod macros;
