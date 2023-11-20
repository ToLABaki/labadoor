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
    fn allow_access(user: String, resource: String);
    fn deny_access(user: Option<String>, resource: Option<String>);

    /// AuthMethod
    fn add_auth_method(user: String, method: String, identifier: String);
    fn del_auth_method(user: Option<String>, method: Option<String>, identifier: Option<String>);

    /// ResourceShortcuts
    fn add_shortcut(user: String, resource: String, shortcut: i8);
    fn del_shortcut(user: Option<String>, resource: Option<String>, shortcut: Option<i8>);

    /// Queries
    fn get_username(method: String, identifier: String) -> Result<String, ()>;
    fn get_resource(username: String, shortcut: i8) -> Result<String, ()>;
    fn is_allowed(username: String, resource: String) -> Result<(), ()>;

    fn del_user(user: String) {
        Self::deny_access(Some(user.clone()), None);
        Self::del_auth_method(Some(user.clone()), None, None);
        Self::del_shortcut(Some(user), None, None);
    }

    fn del_resource(resource: String) {
        Self::deny_access(None, Some(resource.clone()));
        Self::del_shortcut(None, Some(resource), None);
    }

    fn create_user(
        username: String,
        resource: String,
        method: String,
        identifier: String,
        shortcut: i8,
    ) {
        Self::allow_access(username.clone(), resource.clone());
        Self::add_auth_method(username.clone(), method.clone(), identifier.clone());
        Self::add_shortcut(username, resource, shortcut);
    }

    fn auth_user(method: String, identifier: String, shortcut: i8) {
        if let Ok(username) = Self::get_username(method, identifier) {
            if let Ok(resource) = Self::get_resource(username.clone(), shortcut) {
                if Self::is_allowed(username, resource).is_ok() {
                    println!("Open Sesame!");
                }
            }
        }
    }
}
