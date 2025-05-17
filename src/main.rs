use ipkg::dprintln;
use ipkg::modules::messages;
use ipkg::utils::shell::args::{Argument, ArgumentType};
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
    let sub_args: Vec<&Argument> = args[1..].iter().collect();
    enum SubCommand {
        Help,
        Version,
        Unknown,
    }
    let arg_str = command.arg_str.as_str();
    let sub_command: SubCommand = match &command.arg_type {
        ArgumentType::LongOpt => match arg_str {
            "--help" => SubCommand::Help,
            "--version" => SubCommand::Version,
            _ => SubCommand::Unknown,
        },
        ArgumentType::ShortOpt => SubCommand::Unknown,
        ArgumentType::Simple => SubCommand::Unknown,
    };
    match sub_command {
        SubCommand::Help => messages::help(sub_args),
        SubCommand::Version => messages::version(),
        SubCommand::Unknown => messages::unknown(),
    }
    Ok(())
}
