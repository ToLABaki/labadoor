use serde_derive::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
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

impl PartialEq for AuthMethod {
    fn eq(&self, other: &Self) -> bool{
        (self.method == other.method)
            && (self.identifier == other.identifier)
    }
}

impl PartialEq for ResourceShortcuts {
    fn eq(&self, other: &Self) -> bool{
        (self.username == other.username)
            && (self.id == other.id)
    }
}

pub trait ACL {
    /// `Option`s on delete operations mean that all associated data is removed
    /// ACLEntry
    fn allow_access(&self, user: String, resource: String);
    fn deny_access(&self, user: Option<String>, resource: Option<String>);

    /// AuthMethod
    fn add_auth_method(&self, user: String, method: String, identifier: String);
    fn del_auth_method(
        &self,
        user: Option<String>,
        method: Option<String>,
        identifier: Option<String>,
    );

    /// ResourceShortcuts
    fn add_shortcut(&self, user: String, resource: String, shortcut: i8);
    fn del_shortcut(&self, user: Option<String>, resource: Option<String>, shortcut: Option<i8>);

    /// Queries
    fn get_username(&self, method: String, identifier: String) -> Option<String>;
    fn get_resource(&self, username: String, shortcut: i8) -> Option<String>;
    fn is_allowed(&self, username: String, resource: String) -> Option<()>;

    fn del_user(&self, user: String) {
        self.deny_access(Some(user.clone()), None);
        self.del_auth_method(Some(user.clone()), None, None);
        self.del_shortcut(Some(user), None, None);
    }

    fn del_resource(&self, resource: String) {
        self.deny_access(None, Some(resource.clone()));
        self.del_shortcut(None, Some(resource), None);
    }

    fn create_user(
        &self,
        username: String,
        resource: String,
        method: String,
        identifier: String,
        shortcut: i8,
    ) {
        self.allow_access(username.clone(), resource.clone());
        self.add_auth_method(username.clone(), method.clone(), identifier.clone());
        self.add_shortcut(username, resource, shortcut);
    }

    fn auth_user(&self, method: String, identifier: String, shortcut: i8) {
        if let Some(username) = self.get_username(method, identifier) {
            if let Some(resource) = self.get_resource(username.clone(), shortcut) {
                if self.is_allowed(username.clone(), resource).is_some() {
                    println!("Open Sesame! {}", username);
                }
            }
        }
    }
}
