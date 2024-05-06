use teloxide::{
    payloads::{EditMessageTextSetters, SendMessageSetters},
    requests::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::command::BotCommands,
    Bot,
};

use crate::{
    backend::downloader::YoutubeDlp,
    config::Config,
    telegram::{Data, MyDialogue, State},
};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    #[command(description = "Tampilkan pesan ini.")]
    Help,
    #[command(description = "Unggah ulang <url> ke reels anda.")]
    Post(String),
}

pub async fn command_handler(
    bot: Bot,
    message: Message,
    cmd: Command,
    dialogue: MyDialogue,
    config: Config,
) -> anyhow::Result<()> {
    match cmd {
        Command::Help | Command::Start => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .reply_to_message_id(message.id)
                .await?;
        }
        Command::Post(url) => {
            if url.is_empty() {
                bot.send_message(message.chat.id, "penggunaan: `/post <url_video>`")
                    .reply_to_message_id(message.id)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;
            } else {
                let mut text = "• Memproses video".to_string();
                let msg = bot
                    .send_message(message.chat.id, text.clone())
                    .reply_to_message_id(message.id)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;

                text.push_str("\n• Mengambil metadata");
                bot.edit_message_text(message.chat.id, msg.id, text.clone())
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;

                let mut button = vec![
                    vec![InlineKeyboardButton::callback("Lanjutkan", "next_download")],
                    vec![InlineKeyboardButton::callback("Batalkan", "cancel")],
                ];

                let youtube_dl = YoutubeDlp::new(url.clone());
                match youtube_dl.dump_single_json() {
                    Ok(json) => {
                        if json.duration.unwrap_or(0.0) > 90.0 {
                            text.push_str(
                                "```Error\nDurasi video tidak boleh lebih dari 90 detik```",
                            );
                            bot.edit_message_text(message.chat.id, msg.id, text)
                                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                                .await?;
                            return Ok(());
                        }

                        let output_video = format!(
                            "{}/{}.{}",
                            config.ytdlp.download_path,
                            json.id,
                            json.ext.unwrap_or("mp4".to_string())
                        );

                        let mut description = String::new();
                        if let Some(desc) = json.description {
                            description.push_str(&desc);
                            if json.title != description {
                                text.push_str(format!("\n• `{}`", json.title).as_str());
                            }
                            text.push_str(format!("\n```Deskripsi\n{}```", description).as_str());

                            button[0].insert(
                                0,
                                InlineKeyboardButton::callback("Edit Deskripsi", "edit"),
                            );
                            bot.edit_message_text(
                                message.chat.id,
                                msg.id,
                                format!("{}\n• Apakah kamu sudah yakin?", text),
                            )
                            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                            .reply_markup(InlineKeyboardMarkup::new(button))
                            .await?;
                        } else {
                            button[0]
                                .insert(0, InlineKeyboardButton::callback("Tambahkan", "edit"));
                            bot.edit_message_text(
                                message.chat.id,
                                msg.id,
                                format!("{}\n• Video tidak memiliki deskripsi", text),
                            )
                            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                            .reply_markup(InlineKeyboardMarkup::new(button))
                            .await?;
                        }

                        let data = Data {
                            text,
                            url,
                            output: output_video,
                            description,
                            ..Default::default()
                        };
                        dialogue.update(State::Confirm(data)).await?;
                    }
                    Err(e) => {
                        text.push_str(format!("\n```Error\n{}```", e.to_string().trim()).as_str());
                        bot.edit_message_text(message.chat.id, msg.id, text)
                            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                            .await?;
                    }
                }
            }
        }
    }
    Ok(())
}
