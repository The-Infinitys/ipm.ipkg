use ipkg::dprintln;
use ipkg::utils::shell;
fn main() {
    let command_data = shell::args::init();
    dprintln!("{}", command_data);
}
