use std::collections::BTreeMap;

pub type Binary = Vec<String>;

#[derive(Clone, Debug)]
pub struct OpenArgs {
    pub auth: BTreeMap<String, Binary>,
    pub hardware: BTreeMap<String, Binary>,
    pub method: String,
    pub identifier: String,
    pub resource_shortcut: i8,
}

#[derive(Clone, Debug)]
struct AuthResult {
    pub username: String,
    pub resource: String,
}

fn run_bin(bin: Binary) -> Result<String, ()> {
    let mut ret = Err(());
    let mut iter = bin.iter();
    let mut cmd = std::process::Command::new(iter.next().unwrap());
    cmd.args(iter);

    if let Ok(_) = cmd.status() {
        let stdout = cmd.output().unwrap().stdout;
        let str = std::str::from_utf8(stdout.as_slice()).unwrap().trim();
        ret = Ok(String::from(str));
    }
    ret
}

fn run_auth(args: &OpenArgs, bin: Binary) -> Result<AuthResult, ()> {
    let mut ret = Err(());

    let mut auth_bin = bin.clone();
    auth_bin.push(args.method.clone());
    auth_bin.push(args.identifier.clone());
    auth_bin.push(args.resource_shortcut.to_string());
    let output = run_bin(auth_bin);

    if let Ok(out) = output {
        let s: Vec<String> = out.split(',').map(|s| String::from(s)).collect();
        ret = Ok(AuthResult {
            username: s[0].clone(),
            resource: s[1].clone(),
        })
    }
    ret
}

pub fn open(args: OpenArgs) -> Result<(), String> {
    let mut ret = Err("Not authorized!".to_string());

    for (method, binary) in args.auth.iter() {
        let output = run_auth(&args, binary.to_vec());
        if let Ok(user) = output {
            let resource_bin = args.hardware.get(&user.resource).unwrap();
            if let Ok(_) = run_bin(resource_bin.to_vec()) {
                ret = Err("Hardware failure!".to_string());
            }
        }
    }
    ret
}
