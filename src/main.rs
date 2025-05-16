use ipkg::dprintln;
use ipkg::utils::shell;
fn main() -> Result<(), u8> {
    let command_data = shell::args::init();
    dprintln!("{}", command_data);
    let args = command_data.args;
    if args.is_empty() {
        return Ok(());
    }
    Ok(())
}
