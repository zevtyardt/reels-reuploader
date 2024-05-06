//#![allow(dead_code, unused_imports)]

use std::{fs, path::Path, process::exit};
use tokio::runtime;
use which::which;

use crate::config::Config;

mod backend;
mod config;
mod error;
mod message;
mod telegram;

fn main() -> anyhow::Result<()> {
    if which("yt-dlp").is_err() {
        paris::error!("yt-dlp tidak ditemukan, gunakan perintah dibawah untuk menginstall\n\n\tpip install yt-dlp\n");
        exit(0)
    }

    clearscreen::clear()?;
    eprintln!(
        r#"
█▀█ █▀▀ █▀▀ █░░ █▀ ▄▄ █▀█ █▀
█▀▄ ██▄ ██▄ █▄▄ ▄█ ░░ █▀▄ ▄█

█▀█ █▀▀ █░█ █▀█ █░░ █▀█ ▄▀█ █▀▄ █▀▀ █▀█
█▀▄ ██▄ █▄█ █▀▀ █▄▄ █▄█ █▀█ █▄▀ ██▄ █▀▄ v{} 
"#,
        env!("CARGO_PKG_VERSION")
    );

    paris::info!("Memuat config file");
    let config = confy::load_path::<Config>("./config.toml")?;
    if config.telegram.bot_token.is_empty() {
        paris::error!("Bot token belum diatur, silahkan edit file config terlebih dahulu");
        exit(0);
    }

    let video_path = Path::new(&config.ytdlp.download_path);
    if !video_path.is_dir() {
        fs::create_dir(video_path)?;
    }

    paris::info!("Membuat async runtime");
    let runtime = runtime::Builder::new_multi_thread().enable_all().build()?;
    runtime.block_on(async {
        if let Err(e) = telegram::start_bot(config).await {
            paris::error!("Error: {}", e);
        }
    });

    Ok(())
}
