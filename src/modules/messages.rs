use crate::utils::shell::args::Argument;

struct CargoPackageInfo {
    name: &'static str,
    version: &'static str,
    architecture: &'static str,
}
fn get_info() -> CargoPackageInfo {
    CargoPackageInfo {
        name: option_env!("CARGO_PKG_NAME").unwrap_or("ipkg"),
        version: option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        architecture: std::env::consts::ARCH,
    }
}

fn insert_info(text: &'static str) -> String {
    let cargo_package = get_info();
    let replace_list = vec![
        ["name", cargo_package.name],
        ["version", cargo_package.version],
        ["architecture", cargo_package.architecture],
    ];
    let mut text = text.to_string();
    for replaces in replace_list {
        text = text.replace(format!("{{{}}}", replaces[0]).as_str(), replaces[1]);
    }
    text
}

pub fn welcome() {
    let welcome_str = insert_info(include_str!("./messages/welcome.txt"));
    println!("{}", welcome_str);
}

pub fn version() {
    let cargo_package = get_info();
    println!("{} {}", cargo_package.name, cargo_package.version);
}

pub fn help(args: Vec<&Argument>) {
    let help_type: HelpType = if args.is_empty() {
        HelpType::General
    } else {
        match args[0].arg_str.as_str() {
            "install" => HelpType::Install,
            _ => HelpType::General,
        }
    };
    show_help(help_type);
}

enum HelpType {
    General,
    Install,
}
fn get_help_msg(help_type: HelpType) -> String {
    insert_info(match help_type {
        HelpType::General => include_str!("./messages/help/general.txt"),
        HelpType::Install => include_str!("./messages/help/install.txt"),
    })
}
fn show_help(help_type: HelpType) {
    let help_msg = get_help_msg(help_type);
    println!("{}", help_msg);
}
pub fn unknown() {
    eprintln!("unknown help type");
}
