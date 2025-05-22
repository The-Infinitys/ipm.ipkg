use cmd_arg::cmd_arg::get as get_cmd_data;
use cmd_arg::cmd_arg::{Option, OptionType};
use ipkg::dprintln;
use ipkg::modules::{messages, pkg, project, system};
use ipkg::utils::shell::{self, ExitStatus};
fn main() ->Result<(),std::io::Error>{
    let command_data = get_cmd_data();
    dprintln!("{}", command_data);
    let opts = command_data.opts;
    if opts.is_empty() {
        messages::welcome();
        shell::exit(ExitStatus::NoArgs);
    }
    let command = &opts[0];
    let sub_opts: Vec<&Option> = opts[1..].iter().collect();
    enum SubCommand {
        Help,
        Version,
        Project,
        Package,
        Unknown,
        System,
    }
    let opt_str = command.opt_str.as_str();
    let sub_command: SubCommand = match &command.opt_type {
        OptionType::LongOpt => match opt_str {
            "--help" => SubCommand::Help,
            "--version" => SubCommand::Version,
            _ => SubCommand::Unknown,
        },
        OptionType::ShortOpt => match opt_str {
            "-h" => SubCommand::Help,
            "-v" => SubCommand::Version,
            _ => SubCommand::Unknown,
        },
        OptionType::Simple => match opt_str {
            "help" => SubCommand::Help,
            "version" => SubCommand::Version,
            "project" => SubCommand::Project,
            "system" => SubCommand::System,
            "pkg" | "package" => SubCommand::Package,
            _ => SubCommand::Unknown,
        },
    };
    match sub_command {
        SubCommand::Help => messages::help(sub_opts),
        SubCommand::Version => messages::version(),
        SubCommand::Project => project::project(sub_opts),
        SubCommand::System => system::system(sub_opts),
        SubCommand::Package => pkg::pkg(sub_opts),
        SubCommand::Unknown => messages::unknown(),
    }
}
