use std::{fs::File, io::{BufReader, Lines, BufRead}, path::Path, collections::VecDeque, cmp, rc::Rc};

use colored::Colorize;
// use colored::*;
use regex::Regex;

pub fn test() {
    // let data = include_str!("/Users/jonnywong/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/range.rs");
    // let re = Regex::new(r#"trait"#).unwrap();

    // data.lines().into_iter().enumerate().for_each(|(idx, line)| {
    //     let m = re.find(line);
    //     match m {
    //         Some(d) => {
    //             let line_number = format!("L{}:\t", idx);
    //             println!("{} {}", line_number.green().bold(), line)
    //         }
    //         _ => ()
    //     }
    // });
    // todo!()
}

pub struct FileMatcher {
    iter: Lines<BufReader<File>>,
    last_matched_line: usize,
    before_lines: usize,
    after_lines: usize,
    re: Regex,
}

impl FileMatcher {
    fn new(f: impl AsRef<Path>, before: usize, after: usize, re: Regex) -> anyhow::Result<Self> {
        let reader = BufReader::new(File::open(f.as_ref())?);
        let iter = reader.lines();
        Ok(Self {
            iter,
            last_matched_line: 0,
            before_lines: before,
            after_lines: after,
            re,
        })
    }

    fn run(self) -> anyhow::Result<i32> {
        let max_len = cmp::max(self.after_lines, self.before_lines);
        let mut around_lines: VecDeque<Rc<LineWrap>> = VecDeque::with_capacity(max_len);
        let mut result: Vec<LineBlock> = Vec::new();
        let mut last_matched: Rc<LineBlock>;

        // step1: ensure around_lines
        // step2: add -A to last matched
        // step3: regex match
        // step4: add all -B to current match
        for (line, r) in self.iter.enumerate() {
            // step1: ensure around_lines
            let content = r?;
            let lw = Rc::new(LineWrap{line, content});
            around_lines.push_back(lw.clone());
            if around_lines.len() > max_len {
                around_lines.pop_front();
            }
            
            // step2: add -A to last matched
            if let Some(last) = result.last_mut() {
                if last.after.len() < self.after_lines {
                    last.after.push(lw.clone())
                }
            }

            // stemp3: regex match
            self.re.find(&lw.content);
        }
        todo!()
    }
}

struct LineWrap {
    line: usize,
    content: String,
}

struct LineBlock {
    before: Vec<Rc<LineWrap>>,
    medium: Vec<Rc<LineWrap>>,
    after: Vec<Rc<LineWrap>>,
}

fn find_and_record(lw: &mut LineWrap, re: Regex) {
    re.replace_all(&lw.content, "$matched".blue().to_string());
    todo!();
}