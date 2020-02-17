#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Client {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

impl Client {
    pub fn new(bot_id: String, channel_id: String, user_id: String) -> Self {
        Self {
            bot_id,
            channel_id,
            user_id,
        }
    }
}
