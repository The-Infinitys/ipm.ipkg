use std::process::{ExitCode, Termination};
pub mod question;
use std::env;
use std::path::Path;
use std::process::{Command, Output};
#[derive(Debug)]
pub enum ExitStatus {
    Success = 0,
    Failure = 1,
    NoArgs = 2,
    UnknownCommand = 127,
}

impl Termination for ExitStatus {
    fn report(self) -> std::process::ExitCode {
        ExitCode::from(self as u8)
    }
}

pub fn exit(status: ExitStatus) {
    std::process::exit(status as i32);
}
pub fn is_cmd_available(cmd: &str) -> bool {
    let path_env = env::var("PATH");
    match path_env {
        Ok(path_env) => {
            let check_paths = path_env.split(":");
            for check_path in check_paths {
                let check_path = Path::new(check_path).join(cmd);
                if check_path.is_file() {
                    return true;
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    false
}

pub fn username() -> String {
    let output: Output = Command::new("whoami")
        .output()
        .expect("failed to execute process");

    if cfg!(target_os = "windows") {
        let info: String = String::from_utf8(output.stdout).unwrap();
        let username: &str = info.split("\\").collect::<Vec<&str>>()[1];
        String::from(username)
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        let username: String = String::from_utf8(output.stdout).unwrap();
        username
    } else {
        panic!("Error");
    }
}
pub fn hostname() -> String {
    let output: Output = Command::new("hostname")
        .output()
        .expect("failed to execute process");
    let hostname: String = String::from_utf8(output.stdout).unwrap().trim().to_owned();
    hostname
}

pub fn shell_type() -> String {
    env::var("SHELL")
        .unwrap_or("unknown".to_string())
        .to_string()
}
