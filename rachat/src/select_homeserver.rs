use core::pin::Pin;
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use tracing::warn;

#[derive(Default)]
pub struct SelectHomeserverRust {
    pub error_string: QString,
}

impl crate::cxxqt_object::qobject::SelectHomeserver {
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
            }
        });
    }
}
