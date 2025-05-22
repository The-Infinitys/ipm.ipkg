use cmd_arg::cmd_arg::Option;
use cmd_arg::cmd_arg::get as get_cmd_data;
use ipkg::dprintln;
use ipkg::modules::{messages, project, system};
use ipkg::utils::shell::{self, ExitStatus};

fn main() -> Result<(), std::io::Error> {
    let command_data = get_cmd_data();
    dprintln!("{}", command_data);
    let opts = command_data.opts;

    // 引数がない場合は早期リターン
    if opts.is_empty() {
        messages::welcome();
        shell::exit(ExitStatus::NoArgs);
    }

    let command = &opts[0];
    let sub_opts: Vec<&Option> = opts[1..].iter().collect();

    // SubCommand enumの定義はそのまま
    enum SubCommand {
        Help,
        Version,
        Project,
        Package,
        Unknown,
        System,
    }

    let opt_str = command.opt_str.as_str();

    // OptionTypeに関わらず、opt_strで直接マッチング
    let sub_command: SubCommand = match opt_str {
        "--help" | "-h" | "help" => SubCommand::Help,
        "--version" | "-v" | "version" => SubCommand::Version,
        "project" => SubCommand::Project,
        "system" => SubCommand::System,
        "pkg" | "package" => SubCommand::Package,
        _ => SubCommand::Unknown,
    };

    match sub_command {
        SubCommand::Help => messages::help(sub_opts)?,
        SubCommand::Version => messages::version()?,
        SubCommand::Project => project::project(sub_opts)?,
        SubCommand::System => system::system(sub_opts)?,
        SubCommand::Package => {} //pkg::pkg(sub_opts)?,
        SubCommand::Unknown => messages::unknown()?,
    }

    Ok(()) // main関数がResultを返すため、成功を示すOkを返す
}
