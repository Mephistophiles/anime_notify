use anyhow::Result;
use telegram_bot::{Api, ChatId, SendMessage};

pub struct Bot {
    api: Api,
}

impl Bot {
    pub fn new(token: &str) -> Bot {
        let api = Api::new(token);

        Bot { api }
    }

    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<()> {
        let message = SendMessage::new(ChatId::new(chat_id), text);

        self.api.send(message).await?;
        Ok(())
    }
}
