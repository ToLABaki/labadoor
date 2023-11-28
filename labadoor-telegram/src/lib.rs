use teloxide::{prelude::*, utils::command::BotCommands};

pub struct TelegramArgs {
    pub trigger: Vec<String>,
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

fn open(trigger: Vec<String>, param: i64) -> Result<String, String> {
    let a = labadoor_common::OpenBinaryArgs {
        method: "telegram".to_string(),
        identifier: param.to_string(),
        resource_shortcut: 1,
    };
    labadoor_common::run_open(a, trigger)
}

async fn answer(
    bot: Bot,
    trigger: Vec<String>,
    message: Message,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Ping => bot.send_message(message.chat.id, "Pong!").await?,
        Command::Open => match open(trigger, message.chat.id.0) {
            Ok(msg) | Err(msg) => bot.send_message(message.chat.id, msg).await?,
        },
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
    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(answer));
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![args.trigger])
        .build()
        .dispatch()
        .await;
}
