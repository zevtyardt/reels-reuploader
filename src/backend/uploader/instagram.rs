use std::{ffi::OsStr, fs, time::Duration};

use headless_chrome::{protocol::cdp::Network::CookieParam, Browser, LaunchOptionsBuilder};
use teloxide::Bot;

use crate::{config::Config, message::MessageEdit, telegram::Data};

pub async fn post_to_instagram_reels(
    bot: &Bot,
    config: Config,
    data: &mut Data,
) -> anyhow::Result<()> {
    let description = data.description.clone();
    let output = fs::canonicalize(data.output.clone())?;
    let mut msg = MessageEdit::new(bot, data);

    msg.add("• Menginisiasi browser").await?;
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

    msg.add("• Memuat cookies").await?;
    let s = fs::read_to_string(config.cookies.instagram.clone())?;
    let cookies: Vec<CookieParam> = serde_json::from_str(&s)?;
    tab.set_cookies(cookies)?;

    msg.add("• Membuka halaman reels").await?;
    tab.navigate_to("https://www.instagram.com/")?;
    msg.timer(10).await?;

    msg.add("• Mengunggah video").await?;
    let elem = tab.wait_for_xpath("//div[7]/div[1]/span[1]/div[1]/a[1]")?;
    elem.click()?;
    msg.timer(2).await?;

    let input = tab.wait_for_xpath("//input[@type = 'file'][@multiple]")?;
    input.set_input_files(&[&output.to_string_lossy()])?;
    msg.timer(2).await?;

    if tab
        .wait_for_xpath("//span[contains(text(), 'privat')]")
        .is_ok()
    {
        msg.add("```Info\nKarena akun Anda privat, hanya pengikut yang akan melihat reel Anda.```")
            .await?;
        let skip_dialog_button = tab.wait_for_element("._acap")?;
        skip_dialog_button.click()?;
        msg.timer(2).await?;
    }

    msg.add("• Menyiapkan reels").await?;
    for _ in 0..2 {
        let next =
            tab.wait_for_xpath("//div[1]/div[1]/div[1]/div[1]/div[1]/div[3]/div[1]/div[1]")?;
        next.click()?;
        msg.timer(2).await?;
    }

    if !description.is_empty() {
        msg.add("• Menambahkan caption sesuai deskripsi").await?;
        let elem = tab.wait_for_xpath("//p[1]")?;
        elem.click()?;
        tab.type_str(&description)?;
        msg.timer(2).await?;
    }

    msg.add("• Sebentar lagi").await?;
    let button = tab.wait_for_xpath("//div[1]/div[1]/div[1]/div[1]/div[3]/div[1]/div[1]")?;
    button.click()?;
    msg.timer(20).await?;

    msg.add("• Mengambil tautan reels").await?;
    let mut reel = String::new();
    let elem = tab.wait_for_xpath("//div[1]/div[1]/span[1]/img[1]/ancestor::a[@href]")?;
    if let Some(href) = elem.get_attribute_value("href")? {
        let link = format!("https://www.instagram.com{}reels", href);
        tab.navigate_to(&link)?;
        msg.timer(10).await?;
        if let Ok(elem) = tab.wait_for_xpath("//a[contains(., '0')]") {
            reel = if let Some(url) = elem.get_attribute_value("href")? {
                format!("https://www.instagram.com{}", url)
            } else {
                elem.click()?;
                tab.wait_until_navigated()?;
                tab.get_url()
            };
        }
    }

    if !reel.is_empty() {
        msg.add(format!("• Tautan ditemukan [Buka sekarang]({})", reel))
            .await?;
    } else {
        msg.add("• Gagal, reels kamu akan muncul sebentar lagi")
            .await?;
    }

    Ok(())
}
