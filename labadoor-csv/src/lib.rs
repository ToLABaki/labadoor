use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct ACLEntry {
    username: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
struct AuthMethod {
    username: String,
    method: String,
    identifier: String,
}

#[derive(Deserialize, Debug)]
struct ResourceShortcuts {
    username: String,
    resource: String,
    id: i8,
}

fn get_username(method: String, identifier: String) -> Result<String, ()> {
    let mut ret = Err(());
    let file = std::fs::File::open("./auth_methods.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let m: AuthMethod = result.unwrap();
        if m.method == method && m.identifier == identifier {
            ret = Ok(m.username);
            break;
        }
    }
    return ret;
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

fn auth_user(username: String, resource: String) -> Result<(), ()> {
    let mut ret = Err(());
    let file = std::fs::File::open("./acl_entries.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let e: ACLEntry = result.unwrap();
        if e.username == username && e.resource == resource {
            ret = Ok(());
            break;
        }
    }
    return ret;
}

pub fn csv() {
    let args: Vec<String> = std::env::args().collect();
    let method = String::from(&args[1]);
    let identifier = String::from(&args[2]);
    let resource_shortcut = args[3].parse::<i8>().unwrap();

    if let Ok(username) = get_username(method, identifier) {
        if let Ok(resource_name) = get_resource_name(username.clone(), resource_shortcut) {
            if auth_user(username, resource_name).is_ok() {
                println!("Open Sesame!");
            }
        }
    }
}
