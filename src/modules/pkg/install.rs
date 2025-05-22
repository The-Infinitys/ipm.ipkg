use crate::utils::shell;
use crate::utils::shell::ExitStatus;
use cmd_arg::cmd_arg::{Option, OptionType};
use std::env;

pub fn install(args: Vec<&Option>) {
    let mut target_path = String::new();
    for arg in args {
        match arg.opt_type {
            OptionType::Simple => target_path = arg.opt_str.to_owned(),
            _ => continue,
        }
    }
    let target_path = env::current_dir()
        .expect("Failed to get current dir.")
        .join(&target_path);
    if !target_path.exists() {
        eprintln!("Couldn't found target path: {}", target_path.display());
        shell::exit(ExitStatus::Failure);
    }
}
