use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chat {
    channel: String,
    steam_id: String,
    name: String,
    message: String,
}

impl Chat {
    pub fn new(channel: String, steam_id: String, name: String, message: String) -> Chat {
        Chat {
            channel: channel,
            steam_id: steam_id,
            name: name,
            message: message,
        }
    }

    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn steam_id(&self) -> &str {
        &self.steam_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
