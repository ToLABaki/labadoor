use labadoor_acl::{ACLEntry, AuthMethod, ResourceShortcuts, ACL};
use serde::Deserialize;

pub struct CSV {
    pub path: String,
}

impl CSV {
    fn find<T: for<'a> Deserialize<'a> + PartialEq>(&self, needle: T, path: &str) -> Option<T> {
        let p = format!("{}/{}", self.path, path);
        println!("{}", p);
        let file = std::fs::File::open(p).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        reader
            .deserialize()
            .map(|x: Result<T, csv::Error>| x.unwrap())
            .collect::<Vec<T>>()
            .into_iter()
            .find(|x: &T| x == &needle)
    }
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

    /// Queries
    fn get_username(&self, method: String, identifier: String) -> Option<String> {
        let needle = AuthMethod {
            method: method,
            identifier: identifier,
            username: "".to_string(),
        };
        let res = self.find::<AuthMethod>(needle, "auth_methods.csv").unwrap();
        Some(res.username)
    }
    fn get_resource(&self, username: String, id: i8) -> Option<String> {
        let needle = ResourceShortcuts {
            username: username,
            id: id,
            resource: "".to_string(),
        };
        let res = self
            .find::<ResourceShortcuts>(needle, "resource_shortcuts.csv")
            .unwrap();
        Some(res.resource)
    }
    fn is_allowed(&self, username: String, resource: String) -> Option<()> {
        let needle = ACLEntry { username, resource };
        self.find::<ACLEntry>(needle, "acl_entries.csv").unwrap();
        Some(())
    }
}
