use core::fmt;
use std::{
    cmp,
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
    rc::Rc,
};

use crate::utils::is_borrowed;
use anyhow::anyhow;
use colored::Colorize;
use regex::Regex;

pub struct FileMatcher {
    iter: Lines<BufReader<File>>,
    last_matched_line: usize,
    before_lines: usize,
    after_lines: usize,
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
            before_lines: before,
            after_lines: after,
            re,
        })
    }

    pub fn run(self) -> anyhow::Result<Vec<LineBlock>> {
        let max_len = cmp::max(self.after_lines, self.before_lines);
        let mut around_lines: VecDeque<Rc<LineWrap>> = VecDeque::with_capacity(max_len);
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
            let mut lw = Rc::new(LineWrap { line, content });
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
            let replaced = self
                .re
                .replace_all(&lw.content, "$matched".blue().bold().to_string());
            let is_matched = !is_borrowed(&replaced);

            // step4: merge to last matched if needed
            if is_matched {
                lw = Rc::new(LineWrap {
                    line,
                    content: replaced.into_owned(),
                });
                if let Some(last) = result.last_mut() {
                    if last.need_to_merge(lw.line, self.after_lines + self.before_lines) {
                        // this line matched<, last matched exists, need to merge this line match to last matched
                        last.extend_medium_to(&mut around_lines)?;
                        // no need to concern step5, should continue here
                        continue;
                    }
                }
                // step5: add all -B to current match
                // no need to merge this line match to last matched,
                // just push this match to result
                let before = calc_before_lines(around_lines.clone().into(), self.before_lines);
                result.push(LineBlock {
                    before,
                    medium: vec![lw.clone()],
                    after: vec![],
                })
            }
            around_lines.push_back(lw);
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub struct LineWrap {
    line: usize,
    content: String,
}

impl fmt::Display for LineWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line = format!("L{}:\t", self.line).green().bold();
        fmt::Display::fmt(&format!("{}{}\n", line, self.content), f)
    }
}

#[derive(Debug)]
pub struct LineBlock {
    before: Vec<Rc<LineWrap>>,
    medium: Vec<Rc<LineWrap>>,
    after: Vec<Rc<LineWrap>>,
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
        around_lines: &mut VecDeque<Rc<LineWrap>>,
    ) -> anyhow::Result<()> {
        self.after.clear();
        let curr_medium_last_line_wrap = if let Some(last) = self.medium.last() {
            last.to_owned()
        } else {
            return Err(anyhow!(crate::error::FileMatcherError::InvalidLineBlock));
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

    fn fmt_sub_block(sub_block: &Vec<Rc<LineWrap>>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for lw in sub_block.iter().map(|x| x.clone()) {
            lw.fmt(f)?
        }
        Ok(())
    }
}

fn calc_before_lines(around_lines: Vec<Rc<LineWrap>>, before: usize) -> Vec<Rc<LineWrap>> {
    let total = around_lines.len();
    let start = if before > total { 0 } else { total - before };
    let before_lw = &around_lines[start..];
    before_lw.into()
}
