use core::pin::Pin;
use cxx_qt::{Initialize, Threading};
use cxx_qt_lib::QString;
use tracing::{error, instrument, warn};

pub use crate::cxxqt_object::qobject::SelectHomeserver;

#[derive(Default)]
pub struct SelectHomeserverRust {
    pub error_string: QString,
}

impl Initialize for SelectHomeserver {
    #[instrument(skip(self))]
    fn initialize(self: Pin<&mut Self>) {
        if let Err(e) = crate::APP_STATE.set_window_title("Select Homeserver") {
            error!("Failed to set window title: {e:?}");
        }
    }
}

impl SelectHomeserver {
    pub fn on_homeserver_text_changed(self: Pin<&mut Self>, homeserver: QString) {
        let homeserver = homeserver.to_string();
        if !crate::rachat()
            .data_store()
            .is_valid_homeserver_name(&homeserver)
        {
            self.set_error_string(QString::from("Invalid homeserver name"));
        } else {
            self.set_error_string(QString::from(""));
        }
    }
    pub fn select_homeserver(&self, homeserver: QString) {
        let homeserver = homeserver.to_string();
        let thread = self.qt_thread();
        tokio::spawn(async move {
            let data_store = crate::rachat().data_store();
            if let Err(e) = data_store.set_homeserver(&homeserver).await {
                warn!("Failed to set homeserver: {e:?}");
                thread
                    .queue(move |root_window| {
                        let error_msg = format!("Failed to set homeserver: {e}");
                        root_window.set_error_string(QString::from(&error_msg));
                    })
                    .unwrap();
            } else {
                thread
                    .queue(move |root_window| {
                        /*root_window
                        .set_next_url(QUrl::from("qrc:/qt/qml/rs/chir/rachat/qml/main.qml"))*/
                        // TODO
                    })
                    .unwrap();
            }
        });
    }
}
