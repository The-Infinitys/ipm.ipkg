use cmd_arg::cmd_arg::{Option, OptionType};
use std::env;

pub fn install(args: Vec<&Option>) ->Result<(),std::io::Error>{
    let mut target_path = String::new();
    for arg in args {
        match arg.opt_type {
            OptionType::Simple => target_path = arg.opt_str.to_owned(),
            _ => continue,
        }
    }
    let target_path = env::current_dir()?.join(&target_path);
    if !target_path.exists() {
        eprintln!("Couldn't found target path: {}", target_path.display());
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }
    Ok(())
}
