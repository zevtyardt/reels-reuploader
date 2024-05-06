//use std::{ffi::OsStr, fs, time::Duration};

//use headless_chrome::{protocol::cdp::Network::CookieParam, Browser, LaunchOptionsBuilder};
use teloxide::Bot;

use crate::{config::Config, telegram::Data};

pub async fn post_to_instagram_reels(
    _bot: &Bot,
    _config: Config,
    _data: &mut Data,
) -> anyhow::Result<()> {
    anyhow::bail!("unimplemented!")

    /* let id = data.message.clone().unwrap().chat.id;
    let description = data.description.clone();
    let output = fs::canonicalize(data.output.clone())?;

    let mut bot_msg = MessageEdit::new(bot, data);
    bot_msg.add("• Menginisiasi browser").await?;

    let opts = LaunchOptionsBuilder::default()
        .headless(true)
        .idle_browser_timeout(Duration::from_secs(999999999))
        .sandbox(false)
        .args(vec![
            OsStr::new("--mute-audio"),
            OsStr::new("--user-agent=\"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36\""),
        ])
        .build()?;
    let browser = Browser::new(opts)?;
    let tab = browser.new_tab()?;
    bot_msg.add("• Memuat cookies").await?;

    let s = fs::read_to_string(config.facebook_cookies_file.clone())?;
    let cookies: Vec<CookieParam> = serde_json::from_str(&s)?;
    tab.set_cookies(cookies)?;

    Ok(())*/
}
