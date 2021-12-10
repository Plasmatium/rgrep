mod error;
mod folder;
mod matcher;
mod playground;
mod test;
mod utils;

use clap::{crate_version, load_yaml, App};
use colored::*;
use crate::folder::FolderHandler;

fn main() {
    let yaml_config = load_yaml!("cli.yaml");
    let matches = App::from(yaml_config)
        .version(crate_version!())
        .get_matches();

    let mut before = matches.value_of("before").map(
        |b| b.parse::<usize>().expect(&error::ArgsError::InvalidParseNumberArg{
            arg: "before", found: b}.to_string()));
    let mut after = matches.value_of("after").map(
        |a| a.parse::<usize>().expect(&error::ArgsError::InvalidParseNumberArg{
            arg: "after", found: a}.to_string()));
    let center = matches.value_of("center").map(
        |c| c.parse::<usize>().expect(&error::ArgsError::InvalidParseNumberArg{
            arg: "center", found: c}.to_string()));

    let warning = "warning: if --center is presented, --before and --after will be ignored";
    let warning = warning.yellow().bold();
    match (before, after, center) {
        (Some(_), _, None) => (),
        (_, Some(_), None) => (),
        (_, _, Some(_)) => {
            println!("{}", warning);
            before = center;
            after = center;
        },
        _ => (),
    }

    let match_pattern = matches.value_of("PATTERN").expect("unable to get PATTERN");
    let glob_pattern = matches.value_of("INPUT").expect("unable to get INPUT");


    let fh = FolderHandler::new(
        match_pattern.to_string(),
        glob_pattern.to_string(),
        before.unwrap_or_default(), after.unwrap_or_default()
    ).unwrap();

    let results = fh.run().unwrap();
    results.iter().for_each(|(filename, line_blocks)| {
        println!("file: {}", filename);
        println!("--------------------");
        line_blocks.iter().for_each(|lineblock| println!("{}", lineblock));
        println!("\n\n")
    })
}
