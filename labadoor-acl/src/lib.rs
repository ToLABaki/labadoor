use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ACLEntry {
    pub username: String,
    pub resource: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthMethod {
    pub username: String,
    pub method: String,
    pub identifier: String,
}

#[derive(Deserialize, Debug)]
pub struct ResourceShortcuts {
    pub username: String,
    pub resource: String,
    pub id: i8,
}
