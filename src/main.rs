mod error;
mod matcher;
mod playground;
mod test;
mod utils;

use clap::{crate_version, load_yaml, App};
use colored::*;

fn main() {
    let yaml_config = load_yaml!("cli.yaml");
    let matches = App::from(yaml_config)
        .version(crate_version!())
        .get_matches();

    let before = matches.value_of("before");
    let after = matches.value_of("after");
    let center = matches.value_of("center");

    let warning = "warning: if --center is presented, --before and --after will be ignored";
    let warning = warning.yellow().bold();
    match (before, after, center) {
        (Some(_), _, None) => (),
        (_, Some(_), None) => (),
        (None, None, Some(_)) => (),
        _ => println!("{}", warning),
    }
}
