# reels-reuploader

Langkah-langkah untuk setup `reels-reuploader` adalah sebagai berikut:

1. Install dependensi yang diperlukan, yaitu:
   - Python
   - Rust
   - FFmpeg
   - yt-dlp (dapat diinstal dengan perintah `pip install yt-dlp`)
   - chromedriver terbaru [download disini](https://chromedriver.chromium.org/downloads)

2. Buat bot Telegram dan ambil token bot tersebut. Kamu dapat mengikuti langkah-langkah di [sini](https://core.telegram.org/bots/tutorial#obtain-your-bot-token) untuk memperoleh token bot Telegram.

3. Buat file `config.toml` dengan mengacu pada contoh file `config.toml.example`. Pastikan nama file konfigurasi tersebut adalah `config.toml`.

4. Masukkan User ID Telegram kamu ke dalam parameter `allowed_user_id` di file `config.toml`. Hal ini akan membatasi akses ke bot Telegram hanya untuk pengguna dengan User ID tersebut. Untuk memperoleh User ID Telegram kamu, kamu dapat menggunakan [@myidbot](https://t.me/myidbot).

5. Terakhir, untuk mengambil cookies Facebook, kamu dapat menggunakan [ekstensi berikut](https://chrome.google.com/webstore/detail/%E3%82%AF%E3%83%83%E3%82%AD%E3%83%BCjson%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E5%87%BA%E5%8A%9B-for-puppet/nmckokihipjgplolmcmjakknndddifde) di Google Chrome. Pastikan nama file sesuai dengan parameter `facebook_cookies_file`.

Setelah mengikuti langkah-langkah di atas, kamu seharusnya sudah siap menggunakan `reels-reuploader` dengan konfigurasi yang telah disesuaikan. untuk menjalankannya ketik `cargo run`.
