use std::collections::BTreeMap;

pub type Binary = Vec<String>;

#[derive(Clone, Debug)]
pub struct OpenArgs {
    pub auth: BTreeMap<String, Binary>,
    pub hardware: BTreeMap<String, Binary>,
    pub log: Binary,
    pub method: String,
    pub identifier: String,
    pub resource_shortcut: i8,
}

#[derive(Clone, Debug)]
struct AuthResult {
    pub username: String,
    pub resource: String,
}

fn run_auth(args: &OpenArgs, bin: Binary) -> Result<AuthResult, String> {
    let mut ret = Err("".to_string());

    let a = labadoor_common::OpenBinaryArgs {
        method: args.method.clone(),
        identifier: args.identifier.clone(),
        resource_shortcut: args.resource_shortcut,
    };
    let output = labadoor_common::run_open(a, bin);
    if let Ok(out) = output {
        let s: Vec<String> = out.split(',').map(|s| String::from(s)).collect();
        ret = Ok(AuthResult {
            username: s[0].clone(),
            resource: s[1].clone(),
        })
    }
    ret
}

fn run_log(args: &OpenArgs, identifier: String, resource: String) -> Result<String, String> {
    let a = labadoor_common::LogBinaryArgs {
        time: "time.now()".to_string(),
        method: args.method.clone(),
        identifier: identifier,
        resource: resource,
    };
    labadoor_common::run_log(a, args.log.clone())
}

pub fn open(args: OpenArgs) -> Result<(), ()> {
    let mut msg = "Not authorized!";
    let mut ret = Err(());
    let mut identifier = args.identifier.clone();
    let mut resource = "Null".to_string();

    for (_, binary) in args.auth.iter() {
        let output = run_auth(&args, binary.to_vec());
        if let Ok(user) = output {
            identifier = user.username;
            resource = user.resource.clone();
            let resource_bin = args.hardware.get(&user.resource).unwrap();
            if let Ok(_) = labadoor_common::run_bin(resource_bin.to_vec()) {
                msg = "Open sesame!";
                ret = Ok(());
            } else {
                msg = "Hardware failure!";
            }
        }
    }
    println!("{}", msg);
    run_log(&args, identifier, resource).expect("Could not log event");
    ret
}
