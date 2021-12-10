use core::fmt;
use std::{
    cmp,
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
    sync::Arc,
};

use crate::utils::is_borrowed;
use anyhow::anyhow;
use colored::Colorize;
use regex::Regex;

pub struct FileMatcher {
    iter: Lines<BufReader<File>>,
    last_matched_line: usize,
    before: usize,
    after: usize,
    re: Regex,
}

impl FileMatcher {
    pub fn new(
        f: impl AsRef<Path>,
        before: usize,
        after: usize,
        re: Regex,
    ) -> anyhow::Result<Self> {
        let reader = BufReader::new(File::open(f.as_ref())?);
        let iter = reader.lines();
        Ok(Self {
            iter,
            last_matched_line: 0,
            before,
            after,
            re,
        })
    }

    pub fn run(self) -> anyhow::Result<Vec<LineBlock>> {
        let mut before_lines: VecDeque<Arc<LineWrap>> = VecDeque::with_capacity(self.before + 1);
        let mut result: Vec<LineBlock> = Vec::new();

        // step1: ensure around_lines
        // step2: add -A to last matched
        // step3: regex match
        // step4: merge to last matched if needed
        // step5: add all -B to current match
        for (line, r) in self.iter.enumerate() {
            let line = line + 1;
            // step1: ensure around_lines
            let content = r?;
            let mut lw = Arc::new(LineWrap {
                lineno: line,
                content,
                exact_matched: false,
            });
            if before_lines.len() > self.before {
                before_lines.pop_front();
            }

            // step3: regex match
            let replaced = self
                .re
                .replace_all(&lw.content, "$matched".yellow().bold().to_string());
            let is_matched = !is_borrowed(&replaced);

            // step4: merge to last matched if needed
            if is_matched {
                lw = Arc::new(LineWrap {
                    lineno: line,
                    content: replaced.into_owned(),
                    exact_matched: true,
                });
                if let Some(last) = result.last_mut() {
                    if last.need_to_merge(lw.lineno, self.after + self.before) {
                        // this line matched<, last matched exists, need to merge this line match to last matched
                        last.extend_medium_to(before_lines.clone().into(), lw)?;
                        // no need to concern step5, should continue here
                        continue;
                    }
                }
                // step5: add all -B to current match
                // no need to merge this line match to last matched,
                // just push this match to result
                let before = calc_before_lines(before_lines.clone().into(), self.before);
                result.push(LineBlock {
                    before,
                    medium: vec![lw.clone()],
                    after: vec![],
                })
            } else {
                // step2: add -A to last matched
                if let Some(last) = result.last_mut() {
                    if last.after.len() < self.after {
                        last.after.push(lw.clone())
                    }
                }
            }

            before_lines.push_back(lw);
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub struct LineWrap {
    lineno: usize,
    content: String,
    exact_matched: bool,
}

impl fmt::Display for LineWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line = match self.exact_matched {
            true => format!("{}L{}:\t{}\n", "-->".bold(), self.lineno, self.content).blue(),
            false => format!("   L{}:\t{}\n", self.lineno, self.content).green(),
        };
        fmt::Display::fmt(&line, f)
    }
}

#[derive(Debug)]
pub struct LineBlock {
    before: Vec<Arc<LineWrap>>,
    medium: Vec<Arc<LineWrap>>,
    after: Vec<Arc<LineWrap>>,
}

impl fmt::Display for LineBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LineBlock::fmt_sub_block(&self.before, f)?;
        LineBlock::fmt_sub_block(&self.medium, f)?;
        LineBlock::fmt_sub_block(&self.after, f)?;
        Ok(())
    }
}

impl LineBlock {
    fn extend_medium_to(
        &mut self,
        before_lines: Vec<Arc<LineWrap>>,
        curr_matched_line: Arc<LineWrap>,
        // after: usize,
        // before: usize,
    ) -> anyhow::Result<()> {
        let curr_medium_last_line_wrap = self.medium.last().ok_or(anyhow!(
            crate::error::FileMatcherError::InvalidLineBlockMissingMedium
        ))?;
        // need must >= 0 or panic
        let need = curr_matched_line.lineno - curr_medium_last_line_wrap.lineno - 1;
        let before_length = cmp::min(before_lines.len(), need);
        let before_lines = &before_lines[before_lines.len() - before_length..];
        let overlapped = self.after.len() + before_lines.len() - need;
        self.medium.extend_from_slice(&self.after);
        self.medium.extend_from_slice(&before_lines[overlapped..]);
        self.medium.push(curr_matched_line);
        self.after.clear();
        Ok(())
    }

    fn need_to_merge(&self, curr_line: usize, intersect_lines: usize) -> bool {
        if let Some(last) = self.medium.last() {
            curr_line - last.lineno <= intersect_lines + 1
        } else {
            false
        }
    }

    fn fmt_sub_block(sub_block: &Vec<Arc<LineWrap>>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for lw in sub_block.iter().map(|x| x.clone()) {
            lw.fmt(f)?
        }
        Ok(())
    }
}

fn calc_before_lines(around_lines: Vec<Arc<LineWrap>>, before: usize) -> Vec<Arc<LineWrap>> {
    let total = around_lines.len();
    let start = if before > total { 0 } else { total - before };
    let before_lw = &around_lines[start..];
    before_lw.into()
}

/* TEST */
#[test]
fn test_file_matcher() {
    let f = "/Users/jonnywong/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/unwind/src/libunwind.rs";
    let re = Regex::new(r"(?P<matched>unsafe fn)").unwrap();
    let fm = FileMatcher::new(f, 50, 80, re).unwrap();
    let result = fm.run().expect("shit!!");
    for lb in result.iter() {
        println!("{}", lb);
    }
}
