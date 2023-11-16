use matrix_sdk::{
    config::SyncSettings,
    room::Room,
    ruma::{
        events::room::message::{
            MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
            TextMessageEventContent,
        },
        UserId,
    },
    Client,
};

fn open(param: String) {
    use std::process::Command;
    use std::io::{self, Write};
    let mut cmd = Command::new("/usr/local/bin/doorlock");
    cmd.arg("matrix").arg(param);
    let out = cmd.output().expect("Could not run command");
    io::stdout().write_all(&out.stdout).unwrap();
}

async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if let Room::Joined(room) = room {
        let msg_body = match event.content.msgtype {
            MessageType::Text(TextMessageEventContent { body, .. }) => body,
            _ => return,
        };

        if msg_body == "1" {
            open(event.sender.to_string());
            let content = RoomMessageEventContent::text_plain("Open sesame!");
            room.send(content, None).await.unwrap();
        }
        if msg_body == "ping" {
            let content = RoomMessageEventContent::text_plain("Pong!");
            room.send(content, None).await.unwrap();
        }
    }
}

#[tokio::main]
pub async fn matrix(username: String, password: String) {
    let user = <&UserId>::try_from(username.as_str()).unwrap();
    let client = Client::builder()
        .server_name(user.server_name())
        .build()
        .await
        .unwrap();
    client.login_username(user, &password).send().await.unwrap();

    client.sync_once(SyncSettings::default()).await.unwrap();
    client.add_event_handler(on_room_message);
    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;
}
