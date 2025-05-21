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
mod purge;
mod remove;
use super::messages;
use super::package::AuthorAboutData;
use create::{ProjectParams, ProjectTemplateType};

pub fn project(args: Vec<&Option>) {
    if args.is_empty() {
        messages::unknown();
        return;
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
        _ => messages::unknown(),
    }
}

fn project_build(args: Vec<&Option>) {
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

                    let binding = build::BuildShell::from_str(build_shell);
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
                messages::unknown()
            }
        }
    }
    match build::build(build_options) {
        Ok(()) => shell::exit(ExitStatus::Success),
        Err(msg) => {
            eprintln!("Error: {}", msg);
            shell::exit(ExitStatus::Failure)
        }
    }
}

// ---
// ## Project Install Function
fn project_install(args: Vec<&Option>) {
    let mut install_options: install::InstallOptions = InstallOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--shell" | "--sh" => {
                if arg.opt_values.len() == 1 {
                    let build_shell = &arg.opt_values.first().unwrap();

                    let binding = install::InstallShell::from_str(build_shell);
                    match binding {
                        Ok(shell_opt) => {
                            install_options.install_shell = shell_opt;
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            shell::exit(ExitStatus::Failure);
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
                messages::unknown();
            }
        }
    }
    match install::install(install_options) {
        Ok(()) => shell::exit(ExitStatus::Success),
        Err(msg) => {
            eprintln!("Error: {}", msg);
            shell::exit(ExitStatus::Failure);
        }
    }
}

// ---
// ## Project Remove Function
fn project_remove(args: Vec<&Option>) {
    let mut remove_options: remove::RemoveOptions = RemoveOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--shell" | "--sh" => {
                if arg.opt_values.len() == 1 {
                    let build_shell = &arg.opt_values.first().unwrap();

                    let binding = remove::RemoveShell::from_str(build_shell);
                    match binding {
                        Ok(shell_opt) => {
                            remove_options.remove_shell = shell_opt;
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
                eprintln!("Available Options: --shell|--sh");
                messages::unknown();
            }
        }
    }
    match remove::remove(remove_options) {
        Ok(()) => shell::exit(ExitStatus::Success),
        Err(msg) => {
            eprintln!("Error: {}", msg);
            shell::exit(ExitStatus::Failure);
        }
    }
}

// ---
// ## Project Purge Function
fn project_purge(args: Vec<&Option>) {
    let mut purge_options: purge::PurgeOptions = PurgeOptions::default();
    for arg in args {
        match arg.opt_str.as_str() {
            "--shell" | "--sh" => {
                if arg.opt_values.len() == 1 {
                    let build_shell = &arg.opt_values.first().unwrap();

                    let binding = purge::PurgeShell::from_str(build_shell);
                    match binding {
                        Ok(shell_opt) => {
                            purge_options.purge_shell = shell_opt;
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
                eprintln!("Available Options: --shell|--sh");
                messages::unknown();
            }
        }
    }
    match purge::purge(purge_options) {
        Ok(()) => shell::exit(ExitStatus::Success),
        Err(msg) => {
            eprintln!("Error: {}", msg);
            shell::exit(ExitStatus::Failure);
        }
    }
}

// ---
// ## Project Metadata Function
fn project_metadata() {
    if metadata::show_metadata().is_err() {
        eprintln!("Error: failed to get metadata");
        shell::exit(ExitStatus::Failure);
    }
    shell::exit(ExitStatus::Success);
}

// ---
// ## Project Create Function
fn project_create(args: Vec<&Option>) {
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
                            shell::exit(ExitStatus::Failure);
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
            _ => messages::unknown(),
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
                eprintln!("Error: failed to set current dir: {}", &params.project_name);
                shell::exit(ExitStatus::Failure);
            }
            if create::create(&params).is_err() {
                eprintln!("Error: failed to create project: {}", &params.project_name);
                shell::exit(ExitStatus::Failure);
            }
        }
        Err(err) => {
            eprintln!(
                "Error: failed to create dir: {}\nDue to: {}",
                &params.project_name,
                err.kind()
            );
            shell::exit(ExitStatus::Failure);
        }
    }
    shell::exit(ExitStatus::Success);
}
