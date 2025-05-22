use crate::utils::{
    self,
    shell::{self, ExitStatus, question},
};
use build::BuildOptions;
use cmd_arg::cmd_arg::Option;
use install::InstallOptions; // Import InstallOptions
use purge::PurgeOptions; // Import PurgeOptions
use remove::RemoveOptions; // Import RemoveOptions
use std::{env, fs, str::FromStr};
mod build;
mod create;
mod install;
mod metadata;
mod package;
mod purge;
mod remove;
use super::messages;
use super::pkg::AuthorAboutData;
use create::{ProjectParams, ProjectTemplateType};
use std::fmt::{self, Display};
use std::process::Command;
#[derive(Default)]
pub enum ExecShell {
    #[default]
    RBash,
    Bash,
    Zsh,
    Csh,
}
impl FromStr for ExecShell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "csh" => Ok(Self::Csh),
            "rbash" => Ok(Self::RBash),
            _ => Err(format!("Unavailable Shell: {}", s)),
        }
    }
}
impl ExecShell {
    fn generate(&self) -> Command {
        match self {
            Self::RBash => {
                let mut cmd = Command::new("bash");
                cmd.arg("-r");
                cmd
            }
            Self::Bash => Command::new("bash"),
            Self::Zsh => Command::new("zsh"),
            Self::Csh => Command::new("csh"),
        }
    }
}
impl Display for ExecShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecShell::RBash => write!(f, "restricted bash"),
            ExecShell::Bash => write!(f, "bash"),
            ExecShell::Zsh => write!(f, "zsh"),
            ExecShell::Csh => write!(f, "csh"),
        }
    }
}

pub fn project(args: Vec<&Option>) -> Result<(), std::io::Error> {
    if args.is_empty() {
        return messages::unknown();
    }
    let sub_cmd = args.first().unwrap();
    let sub_args: Vec<&Option> = args[1..].to_vec();
    match sub_cmd.opt_str.as_str() {
        "create" | "new" => project_create(sub_args),
        "info" | "metadata" => project_metadata(),
        "build" | "compile" => project_build(sub_args),
        "install" => project_install(sub_args), // Added install command
        "remove" => project_remove(sub_args),   // Added remove command
        "purge" => project_purge(sub_args),     // Added purge command
        "package" | "pkg" => project_package(sub_args),
        _ => messages::unknown(),
    }
}
fn project_package(args: Vec<&Option>) -> Result<(), std::io::Error> {
    let mut package_options: package::PackageOptions = package::PackageOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--target" | "target" => {
                if arg.opt_values.len() == 1 {
                    match package::PackageTarget::from_str(arg.opt_values.first().unwrap()) {
                        Ok(package_target) => package_options.target = package_target,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            shell::exit(ExitStatus::Failure)
                        }
                    }
                }
            }
            _ => {
                eprintln!("Invalid Option: {}", arg.opt_str);
                eprintln!("Available Options: --target");
                return messages::unknown();
            }
        }
    }
    match package::package(package_options) {
        Ok(()) => Ok(()),
        Err(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
    }
}
fn project_build(args: Vec<&Option>) -> Result<(), std::io::Error> {
    let mut build_options: build::BuildOptions = BuildOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--release" => {
                build_options.build_mode = build::BuildMode::Release;
            }
            "--debug" => {
                build_options.build_mode = build::BuildMode::Debug;
            }
            "--shell" | "--sh" => {
                if arg.opt_values.len() == 1 {
                    let build_shell = &arg.opt_values.first().unwrap();

                    let binding = ExecShell::from_str(build_shell);
                    match binding {
                        Ok(shell_opt) => {
                            build_options.build_shell = shell_opt;
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            shell::exit(ExitStatus::Failure);
                        }
                    };
                }
            }
            _ => {
                eprintln!("Unknown Option: {}", arg.opt_str);
                eprintln!("Available Options: --release, --debug ,--shell|--sh");
                return messages::unknown();
            }
        }
    }
    match build::build(build_options) {
        Ok(()) => Ok(()),
        Err(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
    }
}

fn project_install(args: Vec<&Option>) -> Result<(), std::io::Error> {
    let mut install_options: install::InstallOptions = InstallOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--shell" | "--sh" => {
                if arg.opt_values.len() == 1 {
                    let build_shell = &arg.opt_values.first().unwrap();

                    let binding = ExecShell::from_str(build_shell);
                    match binding {
                        Ok(shell_opt) => {
                            install_options.install_shell = shell_opt;
                        }
                        Err(e) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                        }
                    };
                }
            }
            "--local" => {
                install_options.install_mode = install::InstallMode::Local;
            }
            "--global" => {
                install_options.install_mode = install::InstallMode::Global;
            }
            _ => {
                eprintln!("Unknown Option: {}", arg.opt_str);
                eprintln!("Available Options: --global, --local ,--shell|--sh");
                return messages::unknown();
            }
        }
    }
    match install::install(install_options) {
        Ok(()) => Ok(()),
        Err(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
    }
}

fn project_remove(args: Vec<&Option>) -> Result<(), std::io::Error> {
    let mut remove_options: remove::RemoveOptions = RemoveOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--shell" | "--sh" => {
                if arg.opt_values.len() == 1 {
                    let build_shell = &arg.opt_values.first().unwrap();

                    let binding = ExecShell::from_str(build_shell);
                    match binding {
                        Ok(shell_opt) => {
                            remove_options.remove_shell = shell_opt;
                        }
                        Err(e) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                        }
                    };
                }
            }
            _ => {
                eprintln!("Unknown Option: {}", arg.opt_str);
                eprintln!("Available Options: --shell|--sh");
                return messages::unknown();
            }
        }
    }
    match remove::remove(remove_options) {
        Ok(()) => Ok(()),
        Err(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
    }
}

fn project_purge(args: Vec<&Option>) -> Result<(), std::io::Error> {
    let mut purge_options: purge::PurgeOptions = PurgeOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--shell" | "--sh" => {
                if arg.opt_values.len() == 1 {
                    let build_shell = &arg.opt_values.first().unwrap();

                    let binding = ExecShell::from_str(build_shell);
                    match binding {
                        Ok(shell_opt) => {
                            purge_options.purge_shell = shell_opt;
                        }
                        Err(e) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                        }
                    };
                }
            }
            _ => {
                eprintln!("Unknown Option: {}", arg.opt_str);
                eprintln!("Available Options: --shell|--sh");
                return messages::unknown();
            }
        }
    }
    match purge::purge(purge_options) {
        Ok(()) => Ok(()),
        Err(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
    }
}

fn project_metadata() -> Result<(), std::io::Error> {
    if metadata::show_metadata().is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to get metadata",
        ));
    }
    Ok(())
}

fn project_create(args: Vec<&Option>) -> Result<(), std::io::Error> {
    let mut params = ProjectParams {
        project_name: String::new(),
        project_template: ProjectTemplateType::Default,
        author: AuthorAboutData {
            name: String::new(),
            email: String::new(),
        },
    };
    for arg in args {
        match arg.opt_str.as_str() {
            "--project-name" | "--name" | "--package-name" => {
                if arg.opt_values.len() == 1 {
                    params.project_name = arg.opt_values.first().unwrap().to_owned();
                }
            }
            "--template" => {
                if arg.opt_values.len() == 1 {
                    let project_template =
                        ProjectTemplateType::from_str(arg.opt_values.first().unwrap().as_str());
                    match project_template {
                        Ok(project_template) => params.project_template = project_template,
                        Err(e) => {
                            let msg = format!("Error: {}", e);
                            eprintln!("Error: {}", msg);
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
                        }
                    }
                }
            }
            "--author-name" => {
                if arg.opt_values.len() == 1 {
                    params.author.name = arg.opt_values.first().unwrap().to_owned();
                }
            }
            "--author-email" => {
                if arg.opt_values.len() == 1 {
                    params.author.email = arg.opt_values.first().unwrap().to_owned();
                }
            }
            _ => return messages::unknown(),
        }
    }
    if params.project_name.is_empty() {
        params.project_name = question::kebab_loop("Project name: ");
    }
    if params.project_template == ProjectTemplateType::Default {
        params.project_template = ProjectTemplateType::Default;
    }
    if params.author.name.is_empty() {
        params.author.name = shell::username();
    }
    if params.author.email.is_empty() {
        params.author.email = utils::generate_email_address();
    }
    println!("{}", params);
    match fs::create_dir(&params.project_name) {
        Ok(_) => {
            if env::set_current_dir(&params.project_name).is_err() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("failed to set current dir: {}", &params.project_name),
                ));
            }
            if create::create(&params).is_err() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("failed to create project: {}", &params.project_name),
                ));
            }
        }
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "failed to create dir: {}\nDue to: {}",
                    &params.project_name,
                    err.kind()
                ),
            ));
        }
    }
    Ok(())
}
