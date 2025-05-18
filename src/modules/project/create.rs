use super::super::pkg::PackageData;
use super::ProjectParams;
pub fn create(params: ProjectParams) {
    let mut project_data = PackageData::default();
    project_data.about.package.name = params.project_name;
    println!("{}", project_data);
}
