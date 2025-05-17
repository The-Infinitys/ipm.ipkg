use ipkg::dprintln;
use ipkg::modules::{messages, project};
use ipkg::utils::shell::args::{Argument, ArgumentType};
use ipkg::utils::shell::{self, ExitStatus};
fn main() {
    let command_data = shell::args::init();
    dprintln!("{}", command_data);
    let args = command_data.args;
    if args.is_empty() {
        messages::welcome();
        shell::exit(ExitStatus::NoArgs);
    }
    let command = &args[0];
    let sub_args: Vec<&Argument> = args[1..].iter().collect();
    enum SubCommand {
        Help,
        Version,
        Project,
        Unknown,
    }
    let arg_str = command.arg_str.as_str();
    let sub_command: SubCommand = match &command.arg_type {
        ArgumentType::LongOpt => match arg_str {
            "--help" => SubCommand::Help,
            "--version" => SubCommand::Version,
            _ => SubCommand::Unknown,
        },
        ArgumentType::ShortOpt => match arg_str {
            "-h" => SubCommand::Help,
            "-v" => SubCommand::Version,
            _ => SubCommand::Unknown,
        },
        ArgumentType::Simple => match arg_str {
            "help" => SubCommand::Help,
            "version" => SubCommand::Version,
            "project" => SubCommand::Project,
            _ => SubCommand::Unknown,
        },
    };
    match sub_command {
        SubCommand::Help => messages::help(sub_args),
        SubCommand::Version => messages::version(),
        SubCommand::Project => project::project(sub_args),
        SubCommand::Unknown => messages::unknown(),
    }
}
