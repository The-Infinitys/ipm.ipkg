use crate::dprintln;
use crate::{modules::pkg::PackageData, utils::files::is_file_exists};

use std::env;
use std::path::PathBuf;
pub fn get_dir() -> Result<PathBuf, ()> {
    let current_path = env::current_dir().unwrap();
    let mut current_path = current_path.as_path();
    loop {
        let metadata_path = current_path.join("package.yaml");
        dprintln!("{}", metadata_path.to_str().unwrap());
        if is_file_exists(metadata_path.to_str().unwrap()) {
            return Ok(current_path.to_owned());
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
pub fn get_path() -> Result<PathBuf, ()> {
    match get_dir() {
        Ok(dir) => Ok(dir.join("package.yaml")),
        Err(()) => Err(()),
    }
}
pub fn metadata() -> Result<PackageData, ()> {
    let metadata_path = get_path()?;
    let read_data = std::fs::read_to_string(metadata_path.to_str().unwrap());
    if read_data.is_err() {
        eprintln!("Error: Failed to read package.yaml");
        return Err(());
    }
    let read_data = read_data.unwrap();
    match serde_yaml::from_str::<PackageData>(&read_data) {
        Ok(package_data) => Ok(package_data),
        Err(e) => {
            eprintln!("Error: Failed to parse package.yaml: {}", e);
            Err(())
        }
    }
}
pub fn show_metadata() -> Result<(), ()> {
    let read_data = metadata();
    match read_data {
        Ok(package_data) => {
            println!("{}", package_data);
            Ok(())
        }
        Err(_) => {
            eprintln!("Error: Failed to parse package.yaml");
            Err(())
        }
    }
}
