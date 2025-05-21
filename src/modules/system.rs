mod configure;
pub mod dir_path;
use super::messages;
use cmd_arg::cmd_arg::Option;
pub fn system(args: Vec<&Option>) {
    if args.is_empty() {
        messages::unknown();
        return;
    }

    let sub_cmd = args.first().unwrap();
    let sub_args: Vec<&Option> = args[1..].to_vec();
    match sub_cmd.opt_str.as_str() {
        "configure" => configure::configure(sub_args),
        _ => messages::unknown(),
    }
}
