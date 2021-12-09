use regex::Regex;

use crate::matcher::FileMatcher;

#[test]
fn test_file_matcher() {
    let f = "src/matcher.rs";
    let re = Regex::new(r"(?P<matched>Vec)").unwrap();
    let fm = FileMatcher::new(f, 3, 2, re).unwrap();
    let result = fm.run().expect("shit!!");
    for lb in result.iter() {
        println!("{}", lb);
    }
}
