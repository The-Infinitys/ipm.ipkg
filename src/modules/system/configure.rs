mod global;
mod local;
use super::super::messages;
use crate::utils::shell::{self, ExitStatus};
use cmd_arg::cmd_arg::Option;
use std::io::{Error, ErrorKind};
pub fn configure(args: Vec<&Option>) {
    if args.is_empty() {
        messages::unknown();
        return;
    }

    let sub_cmd = args.first().unwrap();
    // let sub_args: Vec<&Option> = args[1..].to_vec();
    let result = match sub_cmd.opt_str.as_str() {
        "local" | "--local" | "-l" => local::configure(),
        "global" | "--global" | "-g" => global::configure(),
        _ => Err(Error::new(ErrorKind::NotFound, "The opt was not found")),
    };
    match result {
        Ok(()) => shell::exit(ExitStatus::Success),
        Err(e) => {
            eprintln!("Error: {}", e);
            shell::exit(ExitStatus::Failure)
        }
    };
}
