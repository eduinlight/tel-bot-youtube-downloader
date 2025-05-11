# Telegram YouTube Downloader Bot

A Telegram bot written in Rust that can download videos and audio from YouTube.

## Features

- `/video [youtube_url]` - Download a video from YouTube
- `/audio [youtube_url]` - Download audio from YouTube
- `/help` - Display available commands

## Prerequisites

- Rust and Cargo
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) (a YouTube-DL fork with additional features)

## Installation

1. Clone this repository:

   ```
   git clone https://github.com/yourusername/tel-bot-youtube-downloader.git
   cd tel-bot-youtube-downloader
   ```

2. Install yt-dlp:

   ```
   # On Debian/Ubuntu
   sudo apt install python3-pip
   pip3 install yt-dlp

   # On Arch Linux
   sudo pacman -S yt-dlp

   # On macOS
   brew install yt-dlp
   ```

3. Create a Telegram bot and get your bot token:

   - Talk to [@BotFather](https://t.me/botfather) on Telegram
   - Use the `/newbot` command to create a new bot
   - Copy the token provided by BotFather

4. Set your bot token as an environment variable:
   ```
   export TELEGRAM_BOT_TOKEN="your_bot_token_here"
   ```

## Building and Running

1. Build the project:

   ```
   cargo build --release
   ```

2. Run the bot:

   ```
   cargo run --release
   ```

   Or directly run the binary:

   ```
   ./target/release/tel-bot-youtube-downloader
   ```

## Usage

1. Start a chat with your bot on Telegram
2. Use the following commands:
   - `/video https://www.youtube.com/watch?v=dQw4w9WgXcQ` - Downloads the video
   - `/audio https://www.youtube.com/watch?v=dQw4w9WgXcQ` - Downloads just the audio
   - `/help` - Shows available commands

## Running as a Service

You can set up the bot to run as a systemd service:

1. Create a systemd service file:

   ```
   sudo nano /etc/systemd/system/youtube-telegram-bot.service
   ```

2. Add the following content:

   ```
   [Unit]
   Description=Telegram YouTube Downloader Bot
   After=network.target

   [Service]
   Type=simple
   User=yourusername
   WorkingDirectory=/path/to/tel-bot-youtube-downloader
   ExecStart=/path/to/tel-bot-youtube-downloader/target/release/tel-bot-youtube-downloader
   Environment="TELEGRAM_BOT_TOKEN=your_bot_token_here"
   Restart=on-failure

   [Install]
   WantedBy=multi-user.target
   ```

3. Enable and start the service:
   ```
   sudo systemctl enable youtube-telegram-bot
   sudo systemctl start youtube-telegram-bot
   ```

## License

MIT

## Disclaimer

This tool is for personal use only. Please respect YouTube's terms of service and copyright laws when using this bot.
