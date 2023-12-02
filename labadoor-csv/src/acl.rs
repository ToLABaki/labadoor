use crate::CSV;
use labadoor_acl::{ACLEntry, AuthMethod, ResourceShortcuts, ACL};

impl ACL for CSV {
    /// ACLEntry
    #[allow(unused_variables)]
    fn allow_access(&self, user: String, resource: String) {
        todo!();
    }
    #[allow(unused_variables)]
    fn deny_access(&self, user: Option<String>, resource: Option<String>) {
        todo!();
    }

    /// AuthMethod
    #[allow(unused_variables)]
    fn add_auth_method(&self, user: String, method: String, identifier: String) {
        todo!();
    }
    #[allow(unused_variables)]
    fn del_auth_method(
        &self,
        user: Option<String>,
        method: Option<String>,
        identifier: Option<String>,
    ) {
        todo!();
    }

    /// ResourceShortcuts
    #[allow(unused_variables)]
    fn add_shortcut(&self, user: String, resource: String, shortcut: i8) {
        todo!();
    }
    #[allow(unused_variables)]
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
        let res = self.find::<AuthMethod>(needle, "auth_methods.csv");
        return if let Some(r) = res {
            Some(r.username)
        } else {
            None
        };
    }
    fn get_resource(&self, username: String, id: i8) -> Option<String> {
        let needle = ResourceShortcuts {
            username: username,
            id: id,
            resource: "".to_string(),
        };
        let res = self.find::<ResourceShortcuts>(needle, "resource_shortcuts.csv");
        return if let Some(r) = res {
            Some(r.resource)
        } else {
            None
        };
    }
    fn is_allowed(&self, username: String, resource: String) -> Option<()> {
        let needle = ACLEntry { username, resource };
        let res = self.find::<ACLEntry>(needle, "acl_entries.csv");
        return if let Some(_) = res { Some(()) } else { None };
    }
}
