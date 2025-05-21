use std::process::{ExitCode, Termination};
pub mod question;
use std::env;
use std::path::Path;
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
