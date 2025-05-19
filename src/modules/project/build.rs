pub struct BuildOptions {
    pub build_mode: BuildMode,
    pub build_shell: BuildShell,
}
impl Default for BuildOptions {
    fn default() -> Self {
        BuildOptions {
            build_mode: BuildMode::default(),
            build_shell: BuildShell::default(),
        }
    }
}
impl Display for Bu
pub enum BuildMode {
    Release,
    Debug,
}
impl Default for BuildMode {
    fn default() -> Self {
        BuildMode::Debug
    }
}
pub enum BuildShell {
    RBash,
}
impl Default for BuildShell {
    fn default() -> Self {
        BuildShell::RBash
    }
}
pub fn build(opts: BuildOptions) -> Result<(), String> {
    println!("{:#?}", opts);
    Ok(())
}
