use std::{thread::sleep, time::Duration};

use teloxide::{payloads::EditMessageTextSetters, requests::Requester, Bot, RequestError};

use crate::telegram::Data;

pub struct MessageEdit<'a> {
    bot: &'a Bot,
    data: &'a mut Data,
}

impl<'a> MessageEdit<'a> {
    pub fn new(bot: &'a Bot, data: &'a mut Data) -> Self {
        Self { bot, data }
    }

    pub async fn add<S: Into<String>>(&mut self, text: S) -> anyhow::Result<(), RequestError> {
        let message = self.data.message.clone().unwrap();
        self.data.text.push('\n');
        self.data.text.push_str(&text.into());
        self.bot
            .edit_message_text(message.chat.id, message.id, self.data.text.clone())
            .disable_web_page_preview(true)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }

    pub async fn timer(&self, time: u64) -> anyhow::Result<(), RequestError> {
        let message = self.data.message.clone().unwrap();

        for t in 0..time {
            let text = format!("{}\n• Menunggu {} detik", self.data.text, time - t);
            self.bot
                .edit_message_text(message.chat.id, message.id, text)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;
            sleep(Duration::from_secs(1));
        }
        self.bot
            .edit_message_text(message.chat.id, message.id, self.data.text.clone())
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        Ok(())
    }
}
