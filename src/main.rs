use regex::Regex;
use std::env;
use std::{collections::HashSet, fs};
use teloxide::{
  prelude::*,
  sugar::request::RequestReplyExt,
  types::{InputFile, ParseMode},
  utils::command::BotCommands,
};
use tokio::process::Command as Cmd;
use tracing::{error, info};
use uuid::Uuid;

#[tokio::main]
async fn main() {
  env::var("ALLOWED_USERS").expect("ALLOWED_USERS environment variable not defined");
  tracing_subscriber::fmt::init();

  info!("Bot init");
  let bot = Bot::from_env();
  Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
  rename_rule = "lowercase",
  description = "These commands are supported:"
)]
enum Command {
  #[command(description = "display this text.")]
  Help,
  #[command(description = "Download video")]
  Video(String),
  #[command(description = "Download audio")]
  Audio(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
  match &msg.from {
    Some(user) => {
      if let Some(username) = &user.username {
        info!(username);
        let allowed_users = env::var("ALLOWED_USERS").unwrap();
        let allowed_users_set: HashSet<String> = allowed_users
          .split(',')
          .map(|s| s.trim().to_string())
          .collect();
        if allowed_users_set.contains(username) {
          match cmd {
            Command::Help => handle_help(&bot, &msg).await?,
            Command::Video(url) => handle_download(&url, false, &bot, &msg).await?,
            Command::Audio(url) => handle_download(&url, true, &bot, &msg).await?,
          }
        }
      }
    }
    _ => {}
  }

  Ok(())
}

async fn handle_help(bot: &Bot, msg: &Message) -> ResponseResult<()> {
  info!("Executing command: HELP");
  bot
    .send_message(msg.chat.id, Command::descriptions().to_string())
    .await?;

  info!("Success command executed: HELP");
  Ok(())
}

async fn handle_download(
  url: &str,
  only_audio: bool,
  bot: &Bot,
  msg: &Message,
) -> ResponseResult<()> {
  let command_name = if only_audio { "AUDIO" } else { "VIDEO" };
  info!("Executing command: {command_name}");
  let mut track_str = format!("Downloading: `{url}`");
  let track_message = bot
    .send_message(msg.chat.id, track_str.clone())
    .parse_mode(ParseMode::MarkdownV2)
    .await?;

  if !is_valid_url(url) {
    error!("Url is invalid: {url}");
    track_str = format!("{}\n❌ Error: Invalid url", track_str.clone());
    bot
      .edit_message_text(msg.chat.id, track_message.id, track_str)
      .parse_mode(ParseMode::MarkdownV2)
      .await?;
  } else {
    let uuid = Uuid::new_v4();
    let file_name = format!("{}.{}", uuid, if only_audio { "mp3" } else { "mp4" });
    let mut cmd = Cmd::new("yt-dlp");
    cmd.arg(url);

    if only_audio {
      cmd.args(["-x", "--audio-format", "mp3", "-o", &file_name]);
    } else {
      cmd.args([
        "-f",
        "bestvideo+bestaudio",
        "--merge-output-format",
        "mp4",
        "-o",
        &file_name,
      ]);
    };

    if let Ok(output) = cmd.output().await {
      if output.status.success() {
        info!("✅ Download succeeded");
        track_str = format!("{}\n✅ Downloading succeeded", track_str.clone());
        bot
          .edit_message_text(msg.chat.id, track_message.id, track_str.clone())
          .parse_mode(ParseMode::MarkdownV2)
          .await?;

        if fs::exists(&file_name).is_ok() {
          match only_audio {
            true => {
              bot
                .send_audio(msg.chat.id, InputFile::file(&file_name))
                .reply_to(msg.id)
                .caption(format!("🎵 Here's your audio!",))
                .await?;
            }
            false => {
              bot
                .send_video(msg.chat.id, InputFile::file(&file_name))
                .reply_to(msg.id)
                .caption(format!("📹 Here's your video!",))
                .await?;
            }
          }

          match fs::remove_file(&file_name) {
            Err(e) => {
              error!("{e}");
            }
            _ => {
              info!("File deleted: {file_name}");
            }
          }
        }
      } else {
        error!("Error yt-dlp failed");
        track_str = format!("{}\n❌ Error yt-dlp failed", track_str.clone());
        bot
          .edit_message_text(msg.chat.id, track_message.id, track_str.clone())
          .parse_mode(ParseMode::MarkdownV2)
          .await?;
      }
    } else {
      error!("Error executing yt-dlp");
      track_str = format!("{}\n❌ Error executing yt-dlp", track_str.clone());
      bot
        .edit_message_text(msg.chat.id, track_message.id, track_str.clone())
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    }
  }

  info!("Success command: {command_name}");
  Ok(())
}

fn is_valid_url(url: &str) -> bool {
  Regex::new(r"^https?://(www\.)?youtube\.com/watch\?v=[\w-]+")
    .map(|r| r.is_match(url))
    .unwrap_or(false)
}
