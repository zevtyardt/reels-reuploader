use std::{ffi::OsStr, fs, thread::sleep, time::Duration};

use headless_chrome::{protocol::cdp::Network::CookieParam, Browser, LaunchOptionsBuilder};
use teloxide::Bot;
use tokio::time::Instant;

use crate::{config::Config, message::MessageEdit, telegram::Data};

pub async fn post_to_facebook_reels(
    bot: &Bot,
    config: Config,
    data: &mut Data,
) -> anyhow::Result<()> {
    // let id = data.message.clone().unwrap().chat.id;
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

    let s = fs::read_to_string(config.cookies.facebook.clone())?;
    let cookies: Vec<CookieParam> = serde_json::from_str(&s)?;
    tab.set_cookies(cookies)?;

    bot_msg.add("• Membuka halaman reels").await?;
    tab.navigate_to("https://www.facebook.com/reels/create")?;
    bot_msg.timer(10).await?;

    bot_msg.add("• Mengunggah video").await?;
    let s = Instant::now();
    let element = tab.wait_for_xpath("//input[@class='x1s85apg']")?;
    element.set_input_files(&[&output.to_string_lossy()])?;

    let mut prev = String::new();
    loop {
        if s.elapsed().as_secs() > 60 {
            anyhow::bail!("```Error\nFile tidak dapat diunggah```");
        }
        let element = tab.find_element_by_xpath("//div[2]/div[1]/div[1]/span[1]")?;
        let text = element.get_inner_text()?;
        if !prev.is_empty() && prev != text {
            bot_msg
                .add(format!("• Selesai dalam: `{:?}`", s.elapsed()))
                .await?;
            break;
        }
        prev = text;
        sleep(Duration::from_secs(1));
    }
    bot_msg.timer(2).await?;

    bot_msg.add("• Menyiapkan reels").await?;
    let button = tab.wait_for_xpath("//div[3]/div[2]/div[1]/div[1]/div[1]/div[1]")?;
    button.click()?;
    bot_msg.timer(2).await?;

    let button = tab.wait_for_xpath("//div[3]/div[2]/div[2]/div[1]/div[1]/div[1]")?;
    button.click()?;
    bot_msg.timer(2).await?;

    if !description.is_empty() {
        bot_msg
            .add("• Menambahkan caption sesuai deskripsi")
            .await?;
        let input = tab.wait_for_element(".x1xb5f1y")?;
        input.click()?;
        tab.type_str(&description)?;
        bot_msg.timer(2).await?;
    }

    bot_msg.add("• Sebentar lagi").await?;
    let publish =
        tab.wait_for_xpath("//div[3]/div[2]/div[2]/div[1]/div[1]/div[1]/div[1]/span[1]/span[1]")?;
    publish.click()?;
    bot_msg.timer(20).await?;

    bot_msg.add("• Mengambil tautan reels").await?;
    tab.navigate_to("https://www.facebook.com/me/reels")?;
    bot_msg.timer(10).await?;

    if let Ok(elem) = tab.wait_for_xpath("//a[.='0']") {
        let url = if let Some(url) = elem.get_attribute_value("href")? {
            format!("https://www.facebook.com{}", url)
        } else {
            elem.click()?;
            tab.wait_until_navigated()?;
            tab.get_url()
        };
        bot_msg
            .add(format!("• Tautan ditemukan [Buka sekarang]({})", url))
            .await?;
    } else {
        bot_msg
            .add("• Gagal, reels kamu akan muncul sebentar lagi")
            .await?;
    }

    Ok(())
}
