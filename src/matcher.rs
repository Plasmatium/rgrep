use std::{fs::File, io::{BufReader, Lines, BufRead}, path::Path, collections::VecDeque, cmp, rc::Rc};

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
    fn new(f: impl AsRef<Path>, before: usize, after: usize, re_str: &str) -> anyhow::Result<Self> {
        let re = Regex::new(re_str)?;
        let reader = BufReader::new(File::open(f.as_ref())?);
        let iter = reader.lines();

        Ok(Self {
            iter,
            last_matched_line: 0,
            before_lines: before,
            after_lines: after,
            re,
        })
        // todo!()
    }

    fn iter(self) -> anyhow::Result<i32> {
        let mut around_lines: VecDeque<LineWrap> = VecDeque::new();
        let max_len = cmp::max(self.after_lines, self.before_lines);
        for (line, r) in self.iter.enumerate() {
            let content = Rc::new(r?);
            let lw = LineWrap{line, content};
            around_lines.push_back(lw);
            if around_lines.len() > max_len {
                around_lines.pop_front();
            }
            
        }
        todo!()
    }
}

#[derive(Clone)]
struct LineWrap {
    line: usize,
    content: Rc<String>,
}

struct LineBlock {
    before: Vec<LineWrap>,
    medium: Vec<LineWrap>,
    after: Vec<LineWrap>,
}