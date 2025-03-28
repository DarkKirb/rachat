//! Macros module

/// Localizes a string with the global localizer
#[macro_export]
macro_rules! loc {
    ($msgid:ident) => {
        $crate::i18n::ඞ::localize(stringify!($msgid), None)
    };
    ($msgid:ident, $($argname: ident = $argval: expr),+) => {
        {
            let mut __loc_args = std::collections::HashMap::new();

            $(
                __loc_args.insert(std::borrow::Cow::Borrowed(stringify!($argname)), $crate::i18n::fluent_bundle::FluentValue::from(&$argval));
            )+

            $crate::i18n::ඞ::localize(stringify!($msgid), Some(&__loc_args))
        }
    }
}

/// Localizes a given message for logging
#[macro_export]
macro_rules! event {
    ($level: expr, $msgid:ident) => {
        $crate::i18n::tracing::event!($level, message = $crate::loc!($msgid), msgid = stringify!($msgid));
    };
    ($level: expr, $msgid:ident, $($argname: ident = $argval: expr),+) => {
        $crate::i18n::tracing::event!($level, message = $crate::loc!($msgid, $($argname = $argval),+), msgid = stringify!($msgid), $($argname = &$argval),+);
    };
}

/// Logs a localized message  with the info level
#[macro_export]
macro_rules! info {
    ($msgid:ident) => {
        $crate::event!($crate::i18n::tracing::Level::INFO, $msgid);
    };
    ($msgid:ident, $($argname: ident = $argval: expr),+) => {
        $crate::event!($crate::i18n::tracing::Level::INFO, $msgid, $($argname = $argval),+);
    };
}

/// Logs a localized message  with the error level
#[macro_export]
macro_rules! error {
    ($msgid:ident) => {
        $crate::event!($crate::i18n::tracing::Level::ERROR, $msgid);
    };
    ($msgid:ident, $($argname: ident = $argval: expr),+) => {
        $crate::event!($crate::i18n::tracing::Level::ERROR, $msgid, $($argname = $argval),+);
    };
}
