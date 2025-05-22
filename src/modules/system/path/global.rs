use std::path::PathBuf;

pub fn packageslist_filepath() -> PathBuf {
    packages_dirpath().join("/list.yaml")
}
pub fn packages_dirpath() -> PathBuf {
    PathBuf::from("/etc/ipkg/packages/")
}
