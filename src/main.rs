use ipkg::dprintln;
use ipkg::modules::welcome;
use ipkg::utils::shell::{self, ExitStatus};
fn main() -> Result<(), ExitStatus> {
    let command_data = shell::args::init();
    dprintln!("{}", command_data);
    let args = command_data.args;
    if args.is_empty() {
        welcome::welcome();
        return Err(ExitStatus::Failure);
    }
    Ok(())
}
