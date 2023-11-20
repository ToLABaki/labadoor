use labadoor_acl::{ACLEntry, AuthMethod, ResourceShortcuts, ACL};
use serde::Deserialize;

pub struct CSV {
    pub path: String,
}

fn get_resource_name(username: String, id: i8) -> Result<String, ()> {
    let mut ret = Err(());
    let file = std::fs::File::open("./resource_shortcuts.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let rs: ResourceShortcuts = result.unwrap();
        if rs.username == username && rs.id == id {
            ret = Ok(rs.resource);
            break;
        }
    }
    return ret;
}

impl ACL for CSV {
    /// ACLEntry
    fn allow_access(&self, user: String, resource: String) {
        todo!();
    }
    fn deny_access(&self, user: Option<String>, resource: Option<String>) {
        todo!();
    }

    /// AuthMethod
    fn add_auth_method(&self, user: String, method: String, identifier: String) {
        todo!();
    }
    fn del_auth_method(
        &self,
        user: Option<String>,
        method: Option<String>,
        identifier: Option<String>,
    ) {
        todo!();
    }

    /// ResourceShortcuts
    fn add_shortcut(&self, user: String, resource: String, shortcut: i8) {
        todo!();
    }
    fn del_shortcut(&self, user: Option<String>, resource: Option<String>, shortcut: Option<i8>) {
        todo!();
    }

    if let Ok(username) = get_username(method, identifier) {
        if let Ok(resource_name) = get_resource_name(username.clone(), resource_shortcut) {
            if auth_user(username, resource_name).is_ok() {
                println!("Open Sesame!");
            }
        }
    }
}
