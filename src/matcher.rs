use std::{fs::File, io::{BufReader, Lines, BufRead}, path::Path, collections::VecDeque, cmp, rc::Rc};

use anyhow::anyhow;
use colored::Colorize;
use regex::Regex;
use crate::utils::is_borrowed;

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

    fn run(self) -> anyhow::Result<()> {
        let max_len = cmp::max(self.after_lines, self.before_lines);
        let mut around_lines: VecDeque<Rc<LineWrap>> = VecDeque::with_capacity(max_len);
        let mut result: Vec<LineBlock> = Vec::new();

        // step1: ensure around_lines
        // step2: add -A to last matched
        // step3: regex match
        // step4: merge to last matched if needed
        // step5: add all -B to current match
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

            // step3: regex match
            let replaced = self.re.replace_all(&lw.content, "$matched".blue().to_string());
            let is_matched = is_borrowed(&replaced);
            
            // step4: merge to last matched if needed
            if is_matched {
                if let Some(last) = result.last_mut() {
                    if last.need_to_merge(lw.line, self.after_lines + self.before_lines) {
                        // this line matched, last matched exists, need to merge this line match to last matched
                        last.extend_medium_to(&mut around_lines)?;
                        // no need to concern step5, should continue here
                        continue
                    }
                } else {
                    // step5: add all -B to current match
                    // no need to merge this line match to last matched,
                    // just push this match to result
                    // TODO: bugs on variable: after
                    let (before, after) = around_lines_to_before_and_after(
                        around_lines.clone().into(), self.before_lines, self.after_lines);
                    result.push(LineBlock {
                        before,
                        medium: vec!(lw),
                        after: vec!(),
                    })
                }
            }
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

impl LineBlock {
    // TODO: not finished
    fn extend_medium_to(&mut self, around_lines: &mut VecDeque<Rc<LineWrap>>) -> anyhow::Result<()> {
        self.after.clear();
        let curr_medium_last_line_wrap = if let Some(last) = self.medium.last() {
            last.to_owned()
        } else {
            return Err(anyhow!(crate::error::FileMatcherError::InvalidLineBlock))
        };
        let last_medium_line = curr_medium_last_line_wrap.line;
        loop {
            match around_lines.pop_front() {
                Some(p) => {
                    if p.line <= last_medium_line {
                        continue;
                    }
                    self.medium.push(p)
                }
                None => break,
            }
        }

        Ok(())
    }

    fn need_to_merge(&self, curr_line: usize, intersect_lines: usize) -> bool {
        if let Some(last) = self.medium.last() {
            curr_line - last.line + 1 <= intersect_lines
        } else {
            false
        }
    }
}

fn around_lines_to_before_and_after(around_lines: Vec<Rc<LineWrap>>, before: usize, after: usize)
-> (Vec<Rc<LineWrap>>, Vec<Rc<LineWrap>>) {
    let total = around_lines.len();
    let before_lw = &around_lines[total - before..];
    let after_lw = &around_lines[..after];
    (before_lw.into(), after_lw.into())
}