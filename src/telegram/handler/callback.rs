use std::path::Path;

use teloxide::{
    payloads::EditMessageTextSetters,
    requests::Requester,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot, RequestError,
};
use url::Url;

use crate::{
    backend::{
        downloader::YoutubeDlp,
        uploader::{facebook::post_to_facebook_reels, instagram::post_to_instagram_reels},
    },
    config::Config,
    telegram::{Data, MyDialogue, State},
};

pub async fn callback_handler(
    bot: Bot,
    cb: CallbackQuery,
    dialogue: MyDialogue,
    config: Config,
    mut data: Data, //    (mut text, url, output, description): (String, String, String, String),
) -> anyhow::Result<(), RequestError> {
    if let Some(message) = cb.message {
        data.message = Some(message.clone());
        if cb.data == Some("edit".to_string()) {
            bot.edit_message_text(
                message.chat.id,
                message.id,
                format!("{}\n• Masukan deskripsi baru: ", data.text),
            )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

            let _ = dialogue.update(State::EditDescription(data)).await;
            return Ok(());
        } else if cb.data == Some("next_download".to_string()) {
            if let Err(e) = download_video(&bot, &mut data, config).await {
                data.text
                    .push_str(format!("\n```Error\n{}```", e.to_string().trim()).as_str());
                bot.edit_message_text(message.chat.id, message.id, data.text.clone())
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;
                return Ok(());
            }
            let _ = dialogue.update(State::Confirm(data)).await;
            return Ok(());
        } else if cb.data == Some("post_instagram".to_string()) {
            data.text.push_str("Instagram\n");
            if let Err(e) = post_to_instagram_reels(&bot, config, &mut data).await {
                data.text.push_str(format!("\n```Error\n{}```", e).as_str());
                bot.edit_message_text(message.chat.id, message.id, data.text.clone())
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;
                return Ok(());
            }
        } else if cb.data == Some("post_facebook".to_string()) {
            data.text.push_str("Facebook\n");
            if let Err(e) = post_to_facebook_reels(&bot, config, &mut data).await {
                data.text.push_str(format!("\n```Error\n{}```", e).as_str());
                bot.edit_message_text(message.chat.id, message.id, data.text.clone())
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;
                return Ok(());
            }
        } else if cb.data == Some("cancel".to_string()) {
            bot.edit_message_text(message.chat.id, message.id, "• Tugas dibatalkan")
                .await?;
            let _ = dialogue.exit().await;
            return Ok(());
        }

        let button = vec![InlineKeyboardButton::url(
            "Donasi Seikhlasnya",
            Url::parse("https://trakteer.id/mzyu/tip").unwrap(),
        )];
        bot.edit_message_text(message.chat.id, message.id, data.text.clone())
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .disable_web_page_preview(true)
            .reply_markup(InlineKeyboardMarkup::new([button]))
            .await?;
    }
    Ok(())
}

pub async fn download_video(bot: &Bot, data: &mut Data, config: Config) -> anyhow::Result<()> {
    let message = data.message.clone().unwrap();
    data.text.push_str("\n• Mulai mengunduh video");
    bot.edit_message_text(message.chat.id, message.id, data.text.clone())
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    let path = Path::new(&data.output);

    if path.is_file() {
        data.text.push_str("\n• Video sudah pernah diunduh");
    } else {
        let youtube_dl = YoutubeDlp::new(data.url.clone());
        let dur = youtube_dl.download(data.output.clone(), config.ytdlp.custom_args)?;

        if path.is_file() {
            data.text
                .push_str(format!("\n• Selesai dalam `{:?}`", dur).as_str());
            data.text
                .push_str(format!("\n• File tersimpan di `{}`", data.output).as_str());
        } else {
            anyhow::bail!("Proses pengunduhan gagal!");
        }
    }

    let button = vec![
        vec![
            InlineKeyboardButton::callback("Instagram", "post_instagram"),
            InlineKeyboardButton::callback("Facebook", "post_facebook"),
        ],
        vec![InlineKeyboardButton::callback("Batalkan", "cancel")],
    ];
    bot.edit_message_text(
        message.chat.id,
        message.id,
        format!("{}\n• Unggah ke:", data.text),
    )
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .reply_markup(InlineKeyboardMarkup::new(button))
    .await?;
    data.text.push_str("\n• Unggah ke: ");

    Ok(())
}

pub async fn edit_desc_handler(
    bot: Bot,
    dialogue: MyDialogue,
    message: Message,
    mut data: Data,
) -> anyhow::Result<(), RequestError> {
    let bot_message = data.message.clone().unwrap();
    let mut spl = data.text.split("```");

    message.text().unwrap().clone_into(&mut data.description);
    data.text = format!(
        "{}```Deskripsi\n{}```• Berhasil mengganti",
        spl.next().unwrap_or(&data.text),
        data.description
    );

    let button = vec![
        vec![
            InlineKeyboardButton::callback("Edit Deskripsi", "edit"),
            InlineKeyboardButton::callback("Lanjutkan", "next_download"),
        ],
        vec![InlineKeyboardButton::callback("Batalkan", "cancel")],
    ];

    bot.edit_message_text(message.chat.id, bot_message.id, data.text.clone())
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(InlineKeyboardMarkup::new(button))
        .await?;
    let _ = dialogue.update(State::Confirm(data)).await;

    Ok(())
}
