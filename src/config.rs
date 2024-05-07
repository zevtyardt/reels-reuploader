use std::path::Path;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct YtDlp {
    pub download_path: String,
    pub custom_args: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Telegram {
    pub allowed_user_id: Vec<u64>,
    pub bot_token: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Cookies {
    pub facebook: String,
    pub instagram: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub cookies: Cookies,
    pub telegram: Telegram,
    pub ytdlp: YtDlp,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cookies: Cookies::default(),
            telegram: Telegram {
                ..Default::default()
            },
            ytdlp: YtDlp {
                download_path: "videos".to_string(),
                ..Default::default()
            },
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        Ok(confy::load_path::<Config>("./config.toml")?)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        // cookies
        if !Path::new(&self.cookies.facebook).is_file() {
            anyhow::bail!(
                "config.cookies.facebook: Fioe tidak ditemukan {:?}",
                self.cookies.facebook
            );
        }
        if !Path::new(&self.cookies.instagram).is_file() {
            anyhow::bail!(
                "config.cookies.instagram: File tidak ditemukan {:?}",
                self.cookies.instagram
            );
        }

        // telegram
        if self.telegram.allowed_user_id.is_empty() {
            anyhow::bail!(
                "config.telegram.allowed_user_id: Berikan setidaknya satu telegram user id"
            );
        }
        if self.telegram.bot_token.is_empty() {
            anyhow::bail!(
                "config.telegram.bot_token: Pastikan token sudah diatur sebelum menjalankan bot"
            );
        }

        Ok(())
    }
}
