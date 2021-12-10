use crate::matcher::{FileMatcher, LineBlock};
use glob::glob;
use regex::Regex;
use rayon::prelude::*;

type MatchedFileResult = (String, Vec<LineBlock>);

pub struct FolderHandler {
    re: Regex,
    before: usize,
    after: usize,
    glob_pattern: String,
}

impl FolderHandler {
    pub fn new(
        match_pattern: String,
        glob_pattern: String,
        before: usize,
        after: usize,
    ) -> anyhow::Result<Self> {
        let match_pattern = format!(r"(?P<matched>{})", match_pattern);
        let re = Regex::new(&match_pattern)?;
        Ok(Self {
            re,
            glob_pattern,
            before,
            after,
        })
    }

    pub fn run(&self) -> anyhow::Result<Vec<MatchedFileResult>> {
        let ret: Vec<MatchedFileResult> = glob(&self.glob_pattern)?
            .par_bridge()
            .map(|entry| -> MatchedFileResult {
                let path = entry.expect("failed to glob file");
                let fm = FileMatcher::new(&path, self.before, self.after, self.re.clone())
                    .expect("failed to create FileMatcher");
                let result = fm.run().expect("failed to run FileMatcher");
                let path = path.into_os_string().into_string().expect("failed to convert path to string");
                (path, result)
            })
            .collect();
        Ok(ret)
    }
}

#[test]
fn test_run() {
    let match_pattern = "use".to_string();
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
