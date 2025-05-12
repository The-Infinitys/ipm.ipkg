use ipkg::utils::shell;

fn main() {
    let command_data = shell::args::init();
    println!("{}", command_data);
}
