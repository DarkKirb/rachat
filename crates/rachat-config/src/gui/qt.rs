//! QT UI Settings

use std::{borrow::Cow, sync::LazyLock};

use serde::{Deserialize, Serialize};

/// The settings for QT
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QtSettings<'a> {
    /// The QML style to use. What is supported here varies by platform.
    ///
    /// Platform Defaults:
    ///
    /// - Linux: Depending on DE either `Breeze`, `Fusion`, `Basic`, or any user-installed style
    /// - macOS: `macOS`
    /// - Windows 11: `FluentWinUI3` for QT 6.8 or later, `Universal` otherwise
    /// - Windows 10: `Universal`
    /// - Windows 8/8.1: `Fusion`
    /// - Windows 7 and earlier: `Windows`
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Cow<'a, str>>,
}

/// The platform QT Settings
pub static PLATFORM_SETTINGS: LazyLock<QtSettings<'static>> = LazyLock::new(|| {
    #[allow(unused_mut, reason = "Conditional code")]
    let mut settings = QtSettings::default();
    #[cfg(windows)]
    'out: {
        use winver::WindowsVersion;
        // Ensure that it is not overwritten in env/arguments
        if std::env::var("QT_QUICK_CONTROLS_STYLE").is_ok() {
            break 'out;
        }
        if std::env::args().any(|x| &x == "-style") {
            break 'out;
        }

        // Obtain Windows Version
        let Some(version) = WindowsVersion::detect() else {
            break 'out;
        };

        //  The default is probably fine on Windows 7 and earlier
        if version < WindowsVersion::new(6, 2, 0) {
            break 'out;
        }

        settings.style = Some(Cow::Borrowed("Fusion"));

        #[cfg(any(
            cxxqt_qt_version_at_least_5_7,
            cxxqt_qt_version_at_least_6,
            cxxqt_qt_version_at_least_7
        ))]
        if version >= WindowsVersion::new(10, 0, 0) {
            // Windows 10 and later
            settings.style = Some(Cow::Borrowed("Universal"));
        }

        #[cfg(any(cxxqt_qt_version_at_least_6_8, cxxqt_qt_version_at_least_7))]
        if version >= WindowsVersion::new(10, 0, 22000) {
            // Windows 11 and later
            settings.style = Some(Cow::Borrowed("FluentWinUI3"));
        }
    }
    settings
});
