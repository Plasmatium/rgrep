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
    let match_pattern = "let".to_string();
    let glob_pattern = "src/*.rs".to_string();
    let before = 3;
    let after = 2;
    let fh = FolderHandler::new(match_pattern, glob_pattern, before, after).unwrap();
    let results = fh.run().unwrap();
    results.iter().for_each(|(filename, line_blocks)| {
        println!("file: {}", filename);
        println!("--------------------");
        line_blocks.iter().for_each(|lineblock| println!("{}", lineblock));
        println!("\n\n")
    })
}
