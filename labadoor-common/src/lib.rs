pub type Binary = Vec<String>;
pub type BinaryResult = Result<String, String>;

pub struct OpenBinaryArgs {
    pub method: String,
    pub identifier: String,
    pub resource_shortcut: i8,
}

pub fn run_bin(bin: Binary) -> BinaryResult {
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

pub fn run_open(args: OpenBinaryArgs, bin: Binary) -> BinaryResult {
    let mut auth_bin = bin.clone();
    auth_bin.push(args.method.clone());
    auth_bin.push(args.identifier.clone());
    auth_bin.push(args.resource_shortcut.to_string());
    run_bin(auth_bin)
}
