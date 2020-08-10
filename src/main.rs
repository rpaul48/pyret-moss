#[macro_use] extern crate lazy_static;
extern crate regex;
#[macro_use] extern crate prettytable;
use std::path::PathBuf;
use crate::fingerprint::Fingerprint;
#[macro_use] mod error;
mod fingerprint;
mod normalize;
mod file_io;
mod phase_i;
mod phase_ii;
mod cli;

// Sub represents a student submission.
// Depending on whether input submissions are directories or
// indiv. files, the dir_name field will be Some or None
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Sub {
    pub dir_name: Option<PathBuf>,
    pub documents: Vec<Doc>
}

// Doc represents a file within a submission.
// Docs are initialized as Unprocessed (contents have not yet been
// read), and become Processed once they have been fingerprinted/line counted
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Doc {
    Unprocessed(PathBuf),                       // filepath (yet to be read)
    Processed(PathBuf, Vec<Fingerprint>, usize) // filepath, fingerprints, # of lines
}

impl Doc {
    // Construct a list of line ranges of all fingerprints in this doc
    // that have the given hash
    pub fn get_lines(&self, hash: i64) -> Vec<(i32, i32)> {
        unimplemented!();
    }
}

fn main() {
    use std::path::Path;
    use std::fs::OpenOptions;
    use gag::Redirect;

    let redirect = false;
    let mut print_redirect;

    let out = Path::new("test-dirs/tinker/output.txt");

    if redirect {
        let log = OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open(out)
            .unwrap();

        print_redirect = Redirect::stdout(log).unwrap();
    }


    use ansi_term::Colour::RGB;
    use ansi_term::Colour::{Blue, Yellow};
    use ansi_term::Style;

    println!("sub1/ and sub2/: {}",
            if !redirect {
                RGB(77, 255, 77).bold().paint("3 matches")
            } else {
                Style::default().paint("3 matches")
            });

    let table = table!(
        ["", Fcbic->"sub1/ (67%)", Fcbic->"sub2/ (24%)"],
        [bc->"1", "doc1.arr lines 3-6, 18-21\ndoc2.arr lines 10-12", "doc1.arr lines 39-44"],
        [bc->"2", "doc1.arr lines 30-34", "doc1.arr lines 14-26"],
        [bc->"3", "doc4.arr line 6\ndoc2.arr lines 1-7", "doc3.arr lines 20-21\ndoc2.arr line 1\ndoc4.arr lines 34-39"]);

    table.printstd();

    println!();
    println!("submission1.arr and submission2.arr: 3 matches");
    let t2 = table!(
        ["", Fcbic->"submission1.arr (67%)", Fcbic->"submission2.arr (24%)"],
        [bc->"1", "lines 3-6, 18-21", "lines 39-44"],
        [bc->"2", "lines 30-34", "lines 14-26"],
        [bc->"3", "line 1-7", "lines 20-21, 34-39"]);
    t2.printstd();
}