use std::process::{ExitCode, Termination};
pub mod question;
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
