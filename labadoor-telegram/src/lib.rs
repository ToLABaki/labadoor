use teloxide::{prelude::*, utils::command::BotCommands};

pub struct TelegramArgs {
    pub token: String,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
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

async fn answer(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Ping => bot.send_message(message.chat.id, "Pong!").await?,
        Command::Open => {
            open(message.chat.id.0);
            bot.send_message(message.chat.id, "Open Sesame!").await?
        }
        Command::Register | Command::Start => {
            let msg = format!("Your Telegram ID is: {}", message.chat.id);
            bot.send_message(message.chat.id, msg).await?
        }
    };

    Ok(())
}

#[tokio::main]
pub async fn telegram(args: TelegramArgs) {
    let bot = Bot::new(args.token);
    Command::repl(bot, answer).await;
}
