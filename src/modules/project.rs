use std::fmt::{Display, Formatter, Result};

use crate::utils::shell::question;

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

fn create_empty_project(params: ProjectParams) {
    println!("{}", params);
}

fn get_params_interactively() -> ProjectParams {
    let project_name = question::kebab_loop("project name: ");
    ProjectParams {
        project_name,
        project_template: ProjectTemplateType::Default,
    }
}

pub fn create_project_interactively() {
    let params = get_params_interactively();
    create_empty_project(params);
}
