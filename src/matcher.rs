use std::{str::Lines, fs, rc::Rc};

use colored::*;
use regex::Regex;

pub fn test() {
    let data = include_str!("/home/jonny/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs");
    let re = Regex::new(r#"trait"#).unwrap();

    data.lines().into_iter().enumerate().for_each(|(idx, line)| {
        let m = re.find(line);
        match m {
            Some(d) => {
                let line_number = format!("L{}:\t", idx);
                println!("{} {}", line_number.green().bold(), line)
            }
            _ => ()
        }
    });
    todo!()
}

pub struct FileMatcher {
    last_matched_line: usize,
    before_lines: u16,
    after_lines: u16,
    re: Regex,
    data: String,
}

impl FileMatcher {
    fn new(f: impl Into<String>, before: u16, after: u16, re_str: &str) -> anyhow::Result<Self> {
        let re = Regex::new(re_str)?;
        let data = fs::read_to_string(f.into())?;
        Ok(Self {
            last_matched_line: 0,
            before_lines: before,
            after_lines: after,
            re,
            data,
        })
    }

    fn iter(&self) -> impl Iterator {
        todo!()
    }
}
