use std::convert::TryFrom;
use matrix_sdk::{
    Client, SyncSettings, Result,
    ruma::{UserId, events::{SyncMessageEvent, room::message::{MessageEventContent, MessageType,  TextMessageEventContent,}}},
    room::Room,
};

fn open(param: String) {
    use std::process::Command;
    use std::io::{self, Write};
    let mut cmd = Command::new("/usr/local/bin/doorlock");
    cmd.arg("matrix").arg(param);
    let out = cmd.output().expect("Could not run command");
    io::stdout().write_all(&out.stdout).unwrap();
}

async fn on_room_message(event: SyncMessageEvent<MessageEventContent>, room: Room) {
    if let Room::Joined(room) = room {
        let msg_body = match event.content.msgtype {
            MessageType::Text(TextMessageEventContent { body, .. }) => body,
            _ => return,
        };

        if msg_body == "1" {
            open(event.sender.to_string());
            let content = MessageEventContent::text_plain("Open sesame!");
            room.send(content, None).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let username = std::env::var("LABADOOR_MATRIX_USERNAME").expect("LABADOOR_MATRIX_USERNAME not set");
    let password = std::env::var("LABADOOR_MATRIX_PASSWORD").expect("LABADOOR_MATRIX_PASSWORD not set");

    let user = UserId::try_from(username)?;
    let client = Client::new_from_user_id(user.clone()).await?;

    client.login(user.localpart(), &password, None, None).await?;
    client.sync_once(SyncSettings::default()).await.unwrap();
    client.register_event_handler(on_room_message).await;
    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;

    Ok(())
}
