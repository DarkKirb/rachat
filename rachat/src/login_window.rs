use core::pin::Pin;

use cxx_qt::{Initialize, Threading};
use tracing::error;

pub use crate::cxxqt_object::qobject::LoginWindow;
use crate::{cxxqt_object::qobject::QString, pages::RachatPages, APP_STATE};

#[derive(Default)]
pub struct LoginWindowRust {
    pub homeserver: QString,
}

impl Initialize for LoginWindow {
    fn initialize(self: Pin<&mut Self>) {
        let thread = self.qt_thread();
        APP_STATE.spawn(|| async move {
            let data_store = crate::rachat().data_store();
            match data_store
                .with_client(|client| async move {
                    let homeserver = client.homeserver();
                    Ok::<_, eyre::Error>(homeserver.host_str().map(String::from))
                })
                .await
            {
                Ok(Some(Some(homeserver))) => {
                    thread.queue(move |window| {
                        window.set_homeserver(QString::from(homeserver.as_str()));
                    })?;
                }
                Ok(Some(None)) | Ok(None) => {
                    error!("Login window shown despite no homeserver selected!");
                    APP_STATE.navigate(RachatPages::SelectHomeserver)?;
                }
                Err(e) => {
                    error!("Error getting homeserver: {}", e);
                    APP_STATE.navigate(RachatPages::SelectHomeserver)?;
                }
            }
            Ok::<(), eyre::Error>(())
        });
    }
}

impl LoginWindow {
    pub fn deselect_homeserver(&self) {
        APP_STATE.spawn(|| async move {
            let data_store = crate::rachat().data_store();
            data_store.reset_homeserver().await?;
            APP_STATE.navigate(RachatPages::SelectHomeserver)?;
            Ok::<(), eyre::Error>(())
        });
    }

    pub fn login(&self, username: QString, password: QString) {
        APP_STATE.spawn(move || async move {
            crate::rachat()
                .data_store()
                .login(username.to_string(), password.to_string())
                .await?;
            Ok(())
        })
    }
}
