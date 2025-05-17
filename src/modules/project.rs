use crate::utils::shell::{self, ExitStatus, args::Argument, question};
use std::fmt::{Display, Formatter, Result};
use std::{env, fs};

use super::messages;

#[derive(PartialEq, Eq)]
enum ProjectTemplateType {
    Default,
}
struct ProjectParams {
    project_name: String,
    project_template: ProjectTemplateType,
}

impl Display for ProjectParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let template = match self.project_template {
            ProjectTemplateType::Default => "default",
        };
        write!(f, "Project: {}\nTemplate: {}", self.project_name, template)
    }
}

pub fn project(args: Vec<&Argument>) {
    if args.is_empty() {
        messages::unknown();
        return;
    }
    let sub_cmd = args.first().unwrap();
    let sub_args: Vec<&Argument> = args[1..].to_vec();
    match sub_cmd.arg_str.as_str() {
        "create" | "new" => project_create(sub_args),
        _ => messages::unknown(),
    }
}

fn project_create(args: Vec<&Argument>) {
    if args.is_empty() {
        messages::unknown();
        return;
    }
    let mut params = ProjectParams {
        project_name: String::new(),
        project_template: ProjectTemplateType::Default,
    };
    for arg in args {
        match arg.arg_str.as_str() {
            "--name" => {
                if arg.arg_values.len() == 1 {
                    params.project_name = arg.arg_values.first().unwrap().to_owned();
                }
            }
            "--template" => {
                if arg.arg_values.len() == 1 {
                    match arg.arg_values.first().unwrap().as_str() {
                        "default" => params.project_template = ProjectTemplateType::Default,
                        _ => messages::unknown(),
                    }
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
    match fs::create_dir(&params.project_name) {
        Ok(_) => {
            if let Err(_) = env::set_current_dir(&params.project_name) {
                eprintln!("Failed to set current dir: {}", &params.project_name);
                shell::exit(ExitStatus::Failure);
            }
            init_project_with_params(params)
        }
        Err(_) => shell::exit(ExitStatus::Failure),
    }
    shell::exit(ExitStatus::Success);
}

fn init_project_with_params(params: ProjectParams) {
    println!("{}", params);
    
}
