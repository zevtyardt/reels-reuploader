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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub facebook_cookies_file: String,
    pub telegram: Telegram,
    pub ytdlp: YtDlp,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            facebook_cookies_file: "./cookies.json".to_string(),
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
