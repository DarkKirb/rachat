//! Page enum for the QML pages

use cxx_qt_lib::QUrl;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RachatPages {
    Root,
    SelectHomeserver,
    Login,
}

impl AsRef<str> for RachatPages {
    fn as_ref(&self) -> &str {
        match *self {
            RachatPages::Root => "qrc:/qt/qml/rs/chir/rachat/qml/root.qml",
            RachatPages::SelectHomeserver => "qrc:/qt/qml/rs/chir/rachat/qml/select-homeserver.qml",
            RachatPages::Login => "qrc:/qt/qml/rs/chir/rachat/qml/login.qml",
        }
    }
}

impl From<RachatPages> for QUrl {
    fn from(value: RachatPages) -> Self {
        QUrl::from(value.as_ref())
    }
}
