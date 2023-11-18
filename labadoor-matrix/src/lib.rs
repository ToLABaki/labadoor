use matrix_sdk::{
    config::SyncSettings,
    room::Room,
    ruma::{
        events::{
            room::message::{MessageType, RoomMessageEventContent, TextMessageEventContent},
            OriginalSyncMessageLikeEvent,
        },
        UserId,
    },
    Client,
};

fn open(param: String) {
    use std::io::{self, Write};
    use std::process::Command;
    let mut cmd = Command::new("/usr/bin/labadoor-wrapper");
    cmd.arg("matrix").arg(param);
    let out = cmd.output().expect("Could not run command");
    io::stdout().write_all(&out.stdout).unwrap();
}

async fn on_room_message(
    event: OriginalSyncMessageLikeEvent<RoomMessageEventContent>,
    room: Room,
    client: Client,
) {
    if let Room::Joined(room) = room {
        let msg_body = match event.content.msgtype {
            MessageType::Text(TextMessageEventContent { body, .. }) => body,
            _ => return,
        };
        if event.sender == client.user_id().unwrap() {
            return;
        }

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

async fn client_login(username: String, password: String, device_id: Option<String>) -> Client {
    let user = <&UserId>::try_from(username.as_str()).unwrap();
    let client = Client::builder()
        .server_name(user.server_name())
        .sled_store("./sled_store", None)
        .unwrap()
        .build()
        .await
        .unwrap();

    let mut login_builder = client.login_username(user, &password);
    let d_id = &device_id.as_ref();
    if device_id.is_some() {
        login_builder = login_builder.device_id(d_id.unwrap());
    }
    login_builder.send().await.unwrap();
    if device_id.is_none() {
        println!("Logged in with a new device id: \"{}\"; you can save it in your configuration so we can use it next time.", client.device_id().unwrap());
    }

    client
}

#[tokio::main]
pub async fn matrix(username: String, password: String, device_id: Option<String>) {
    let client = client_login(username, password, device_id).await;
    client.sync_once(SyncSettings::default()).await.unwrap();
    client.add_event_handler(on_room_message);
    client.sync(SyncSettings::default()).await.unwrap();
}
