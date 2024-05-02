use teloxide::{
    dispatching::{
        dialogue::{self, Dialogue, InMemStorage},
        Dispatcher, UpdateFilterExt,
    },
    payloads::EditMessageTextSetters,
    requests::Requester,
    respond,
    types::{CallbackQuery, Message, Update, UserId},
    Bot,
};

use crate::{
    config::Config,
    error::CustomLoggingErrorHandler,
    telegram::handler::{
        callback::{callback_handler, edit_desc_handler},
        command::{command_handler, Command},
    },
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Debug, Default)]
pub struct Data {
    pub text: String,
    pub url: String,
    pub output: String,
    pub description: String,
    pub message: Option<Message>,
}

#[derive(Clone, Default, Debug)]
enum State {
    #[default]
    Start,
    Confirm(Data),
    EditDescription(Data),
}

mod handler;

pub async fn start_bot(config: Config) -> anyhow::Result<()> {
    paris::info!("Menginisiasi telegram client");
    let bot = Bot::new(config.telegram.bot_token.clone());
    bot.get_me().await?;

    for userid in &config.telegram.allowed_user_id {
        bot.send_message(UserId(*userid), "bot is online!").await?;
    }

    let command_handler = teloxide::filter_command::<Command, _>()
        .filter(|config: Config, message: Message| {
            config
                .telegram
                .allowed_user_id
                .contains(&(message.chat.id.0 as u64))
        })
        .branch(dptree::endpoint(
            |bot: Bot, message: Message, cmd: Command, dialogue: MyDialogue, config: Config| async move {
                paris::info!(
                    "Memproses pesan dari @{}: {}",
                    message.chat.username().unwrap(),
                    message.text().unwrap()
                );

                if let Err(e) = command_handler(bot, message, cmd, dialogue, config).await {
                    paris::error!("{}", e);
                }
                respond(())
            },
        ));

    let callback_handler = Update::filter_callback_query()
        .branch(dptree::case![State::Confirm(data)].endpoint(callback_handler))
        .branch(dptree::endpoint(|bot: Bot, cb: CallbackQuery| async move {
            if let Some(message) = cb.message {
                bot.edit_message_text(
                    message.chat.id,
                    message.id,
                    "```Error\nCallback data tidak ditemukan!```",
                )
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;
            }
            respond(())
        }));

    let message_handler = Update::filter_message()
        .filter(|config: Config, message: Message| {
            config
                .telegram
                .allowed_user_id
                .contains(&(message.chat.id.0 as u64))
        })
        .branch(dptree::case![State::EditDescription(data)].endpoint(edit_desc_handler))
        .branch(command_handler);

    let schema = dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_handler);

    paris::info!("Menjalankan bot, tekan Ctrl-C untuk menghentikan");
    Dispatcher::builder(bot, schema)
        .default_handler(|update| async move { paris::warn!("Unhandled update: {:?}", update) })
        .error_handler(CustomLoggingErrorHandler::new())
        .dependencies(dptree::deps![config, InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
