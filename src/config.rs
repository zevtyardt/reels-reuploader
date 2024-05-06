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

/*impl Config {
    pub fn load() -> anyhow::Result<Self> {
        Ok(confy::load_path::<Config>("./config.toml")?)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}*/
