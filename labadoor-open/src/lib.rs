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

fn run_bin(bin: Binary) -> Result<String, String> {
    use std::{
        io::Read,
        process::{Command, Stdio},
    };

    let mut ret = Err("".to_string());
    let mut iter = bin.iter();
    let mut cmd = Command::new(iter.next().unwrap());
    cmd.args(iter);

    let mut child = cmd.stdout(Stdio::piped()).spawn().unwrap();
    let mut success = child.wait().unwrap().success();

    let mut s = String::from("");
    child.stdout.unwrap().read_to_string(&mut s).unwrap();
    s = String::from(s.trim());

    ret = if success { Ok(s) } else { Err(s) };
    ret
}

fn run_auth(args: &OpenArgs, bin: Binary) -> Result<AuthResult, String> {
    let mut ret = Err("".to_string());

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

pub fn open(args: OpenArgs) -> Result<(), ()> {
    let mut msg = "Not authorized!";
    let mut ret = Err(());

    for (method, binary) in args.auth.iter() {
        let output = run_auth(&args, binary.to_vec());
        if let Ok(user) = output {
            let resource_bin = args.hardware.get(&user.resource).unwrap();
            if let Ok(_) = run_bin(resource_bin.to_vec()) {
                msg = "Open sesame!";
                ret = Ok(());
            } else {
                msg = "Hardware failure!";
            }
        }
    }
    println!("{}", msg);
    ret
}
