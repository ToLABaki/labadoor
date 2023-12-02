use matrix_sdk::{
    config::SyncSettings,
    room::Room,
    ruma::{
        events::{
            room::member::RoomMemberEventContent,
            room::message::{MessageType, RoomMessageEventContent, TextMessageEventContent},
            OriginalSyncMessageLikeEvent, StrippedStateEvent,
        },
        UserId,
    },
    Client,
};

pub struct MatrixArgs {
    pub trigger: Binary,
    pub username: String,
    pub password: String,
    pub device_id: Option<String>,
}

fn open(trigger: Vec<String>, param: String) -> BinaryResult {
    let a = labadoor_common::OpenBinaryArgs {
        method: "matrix".to_string(),
        identifier: param,
        resource_shortcut: 1,
    };
    labadoor_common::run_open(a, trigger)
}

async fn on_room_message(
    event: OriginalSyncMessageLikeEvent<RoomMessageEventContent>,
    room: Room,
    client: Client,
    bin: Ctx<Binary>,
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
            let msg = match open(bin.clone(), event.sender.to_string()) {
                Ok(msg) | Err(msg) => msg,
            };
            let content = RoomMessageEventContent::text_plain(msg);
            room.send(content, None).await.unwrap();
        }
        if msg_body == "ping" {
            let content = RoomMessageEventContent::text_plain("Pong!");
            room.send(content, None).await.unwrap();
        }
    }
}

async fn on_room_invite(event: StrippedStateEvent<RoomMemberEventContent>, room: Room) {
    use matrix_sdk::ruma::events::room::member::MembershipState::Invite;
    if event.content.membership == Invite {
        if let Room::Invited(r) = room {
            r.accept_invitation().await.unwrap();
        }
    }
}

async fn client_login(username: String, password: String, device_id: Option<String>) -> Client {
    let user = <&UserId>::try_from(username.as_str()).unwrap();
    let path = "/etc/labadoor/sled_store";
    let client = Client::builder()
        .server_name(user.server_name())
        .sled_store(path, None)
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
pub async fn matrix(args: MatrixArgs) {
    let client = client_login(args.username, args.password, args.device_id).await;
    client.sync_once(SyncSettings::default()).await.unwrap();
    client.add_event_handler(on_room_message);
    client.add_event_handler(on_room_invite);
    client.add_event_handler_context(args.trigger);
    client.sync(SyncSettings::default()).await.unwrap();
}
