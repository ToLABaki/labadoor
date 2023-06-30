use teloxide::{prelude::*, utils::command::BotCommand};
use std::error::Error;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Ping")]
    Ping,
    #[command(description = "Open")]
    Open,
    #[command(description = "Register")]
    Register,
    #[command(description = "Start")]
    Start,
}

fn open(param: i64) {
    use std::process::Command;
    use std::io::{self, Write};
    let mut cmd = Command::new("/usr/local/bin/doorlock");
    cmd.arg("telegram").arg(param.to_string());
    let out = cmd.output().expect("Could not run command");
    io::stdout().write_all(&out.stdout).unwrap();
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Ping => cx.answer("Pong!").await?,
        Command::Open => {
            open(cx.update.chat_id());
            cx.answer("Open sesame!").await?
        },
        Command::Register | Command::Start => {
            let msg = format!("Your Telegram ID is: {}", cx.update.chat_id());
            cx.answer(msg).await?
        },
    };

    Ok(())
}

#[tokio::main]
pub async fn telegram(token: String) {
    teloxide::enable_logging!();
    log::info!("Starting labadoor Telegram bot...");
    let bot = Bot::new(token).auto_send();
    let bot_name: String = "Labadoor Telegram bot".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
