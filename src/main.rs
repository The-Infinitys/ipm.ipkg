use ipkg::dprintln;
use ipkg::modules::messages;
use ipkg::utils::shell::{self, ExitStatus};
fn main() -> Result<(), ExitStatus> {
    let command_data = shell::args::init();
    dprintln!("{}", command_data);
    let args = command_data.args;
    if args.is_empty() {
        messages::welcome();
        return Err(ExitStatus::NoArgs);
    }
    let command = &args[0];
    match command {
        "--help" => messages::help("general"),
        "--version" => messages::version(),
        _ => messages::error("unknown_command"),
    }
    Ok(())
}
