use crate::utils::shell::{self, ExitStatus, question};
use cmd_arg::cmd_arg::Option;
use std::fmt::{Display, Formatter, Result};
use std::{env, fs};
mod create;
mod metadata;
use super::messages;
use super::pkg::AuthorAboutData;
#[derive(PartialEq, Eq)]
enum ProjectTemplateType {
    Default,
}
struct ProjectParams {
    project_name: String,
    project_template: ProjectTemplateType,
    author: AuthorAboutData,
}

impl Display for ProjectParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let template = match self.project_template {
            ProjectTemplateType::Default => "default",
        };
        write!(f, "Project: {}\nTemplate: {}", self.project_name, template)
    }
}

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
        _ => messages::unknown(),
    }
}

fn project_metadata() {
    if metadata::metadata().is_err() {
        eprintln!("Error: failed to get metadata");
        shell::exit(ExitStatus::Failure);
    }
    shell::exit(ExitStatus::Success);
}

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
            "--project-name" => {
                if arg.opt_values.len() == 1 {
                    params.project_name = arg.opt_values.first().unwrap().to_owned();
                }
            }
            "--template" => {
                if arg.opt_values.len() == 1 {
                    match arg.opt_values.first().unwrap().as_str() {
                        "default" => params.project_template = ProjectTemplateType::Default,
                        _ => messages::unknown(),
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
        params.author.name = question::kebab_loop("Author name: ");
    }
    if params.author.email.is_empty() {
        params.author.email = question::email_loop("Author email: ");
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
