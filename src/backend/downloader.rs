use std::{
    env::current_dir,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use serde::Deserialize;
use which::which;

#[derive(Deserialize, Debug)]
pub struct YoutubeValue {
    pub title: String,
    pub id: String,
    pub duration: Option<f64>,
    pub description: Option<String>,
    pub ext: Option<String>,
}

pub struct YoutubeDlp {
    url: String,
}

impl YoutubeDlp {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn dump_single_json(&self) -> anyhow::Result<YoutubeValue> {
        let binary_path = which("yt-dlp")?;
        let mut cmd = Command::new(binary_path);
        cmd.current_dir(&current_dir()?)
            .env("LC_ALL", "en_US.UTF-8")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd.args([
            self.url.clone(),
            "--dump-single-json".to_string(),
            "--no-warning".to_string(),
        ]);

        let child = cmd.spawn()?;
        let output = child.wait_with_output()?;

        if let Ok(json) = serde_json::from_slice::<YoutubeValue>(&output.stdout) {
            Ok(json)
        } else {
            let stderr = output.stderr.to_vec();
            let mut s = String::from_utf8_lossy(&stderr).to_string();
            if s.is_empty() {
                s.push_str("Internal server error")
            }
            anyhow::bail!(s);
        }
    }

    pub fn download(&self, output: String, custom_args: Vec<String>) -> anyhow::Result<Duration> {
        let s = Instant::now();
        let binary_path = which("yt-dlp")?;
        let mut cmd = Command::new(binary_path);
        cmd.current_dir(&current_dir()?)
            .env("LC_ALL", "en_US.UTF-8")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd.args([
            self.url.clone(),
            "--continue".to_string(),
            "--no-warning".to_string(),
            "--output".to_string(),
            output,
        ]);

        if which("ffmpeg").is_ok() {
            cmd.args(["-S".to_string(), "ext".to_string()]);
        }

        if !custom_args.is_empty() {
            for arg in custom_args {
                cmd.arg(arg);
            }
        }

        let child = cmd.spawn()?;
        let output = child.wait_with_output()?;
        if !output.stderr.is_empty() {
            let stderr = output.stderr.to_vec();
            let err = String::from_utf8_lossy(&stderr).to_string();
            anyhow::bail!(err);
        }
        Ok(s.elapsed())
    }
}
