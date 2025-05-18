use crate::dprintln;
use crate::{modules::pkg::PackageData, utils::files::is_file_exists};
use std::env;
pub fn metadata() -> Result<(), ()> {
    let current_path = env::current_dir().unwrap();
    let mut current_path = current_path.as_path();
    loop {
        let metadata_path = current_path.join("package.yaml");
        dprintln!("{}", metadata_path.to_str().unwrap());
        if is_file_exists(metadata_path.to_str().unwrap()) {
            dprintln!("Found package.yaml");
            let read_data = std::fs::read_to_string(metadata_path.to_str().unwrap());
            if read_data.is_err() {
                eprintln!("Error: Failed to read package.yaml");
                return Err(());
            }
            let read_data = read_data.unwrap();
            let read_data = serde_yaml::from_str::<PackageData>(&read_data);
            match read_data {
                Ok(package_data) => {
                    println!("{}", package_data);
                    return Ok(());
                }
                Err(_) => {
                    eprintln!("Error: Failed to parse package.yaml");
                    return Err(());
                }
            }
        } else {
            dprintln!("Not found package.yaml");
            let next_path = current_path.parent();
            if next_path.is_none() {
                eprintln!("Error: package.yaml not found");
                return Err(());
            } else {
                current_path = next_path.unwrap();
            }
        }
    }
}
