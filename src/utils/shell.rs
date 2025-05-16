use std::process::{ExitCode, Termination};

pub mod args;
pub mod question;
#[derive(Debug)]
pub enum ExitStatus {
    Success = 0,
    Failure = 1,
}

impl Termination for ExitStatus {
    fn report(self) -> std::process::ExitCode {
        return ExitCode::from(self as u8);
    }
}
