use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct SelectHomeserverRust;

impl crate::cxxqt_object::qobject::SelectHomeserver {
    pub fn select_homeserver(&self, homeserver: QString) {
        let homeserver = homeserver.to_string();
        tokio::spawn(async move {
            let data_store = crate::rachat().data_store();
            data_store.set_homeserver(&homeserver).await.unwrap();
        });
    }
}
