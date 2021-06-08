use super::api::AsyncApiWrapper;
use frankenstein::Update;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::RwLock;
use tokio::time::Duration;
use frankenstein::EditMessageResponse::Message;

enum State {
    None,
}

pub struct Dispatcher {
    api: AsyncApiWrapper,
    user_states: RwLock<HashMap<isize, State>>,
}

impl Dispatcher {
    pub fn new(api: AsyncApiWrapper) -> Self {
        Self {
            api,
            user_states: RwLock::new(HashMap::new()),
        }
    }

    pub fn dispatch(&self, update: Update) {
        if let Some(message) = &update.message {
            if let (Some(user), Some(text)) = (&message.from, &message.text) {
                self.dispatch_unpacked(user.id, text.to_string());
            } else {
                log::warn!("Failed to unpack message.");
            }
        } else {
            log::debug!("Received non text message while expected.");
        }
    }

    fn dispatch_unpacked(&self, user_id: isize, text: String) {
        println!("id: {}, text: {}", user_id, text);
    }
}
