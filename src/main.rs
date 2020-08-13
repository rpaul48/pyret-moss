#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prettytable;
#[macro_use] mod error;
extern crate regex;
use std::path::PathBuf;
use crate::fingerprint::Fingerprint;
mod fingerprint;
mod normalize;
mod file_io;
mod phase_i;
mod phase_ii;
mod cli;
mod results;
mod io_redirect;

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
    Unprocessed(PathBuf),
    Processed(PathBuf, Vec<Fingerprint>)
}

fn main() {
    use crate::results::render_results;
    use crate::phase_ii::SubPair;
    use crate::cli::SubFileMode;
    use std::path::{Path, PathBuf};
    use std::collections::HashSet;

    let a = Sub {
        dir_name: Some(PathBuf::from("subs/sub1")),
        documents: vec![
            Doc::Processed(PathBuf::from("subs/sub1/doc1.arr"), vec![
                Fingerprint { hash: 17, lines: (1, 3) },
                Fingerprint { hash: 20, lines: (5, 5) },
                Fingerprint { hash: 17, lines: (6, 10) },
                Fingerprint { hash: 11, lines: (10, 11) },
                Fingerprint { hash: 11, lines: (12, 15) }
            ]),
            Doc::Processed(PathBuf::from("subs/sub1/doc2.arr"), vec![
                Fingerprint { hash: 51, lines: (21, 24) },
                Fingerprint { hash: 20, lines: (25, 30) },
                Fingerprint { hash: 17, lines: (44, 57) }
            ])
        ]
    };
    let b = Sub {
        dir_name: Some(PathBuf::from("subs/sub2/")),
        documents: vec![
            Doc::Processed(PathBuf::from("subs/sub2/doc1.arr"), vec![
                Fingerprint { hash: 11, lines: (5, 5) },
                Fingerprint { hash: 17, lines: (8, 12) },
                Fingerprint { hash: 40, lines: (12, 12) },
                Fingerprint { hash: 11, lines: (17, 30) },
                Fingerprint { hash: 33, lines: (29, 34) }
            ]),
            Doc::Processed(PathBuf::from("subs/sub2/doc2.arr"), vec![
                Fingerprint { hash: 12, lines: (3, 4) },
                Fingerprint { hash: 28, lines: (4, 4) },
                Fingerprint { hash: 20, lines: (8, 10) }
            ])
        ]
    };
    let c = Sub {
        dir_name: Some(PathBuf::from("subs/sub3/")),
        documents: vec![
            Doc::Processed(PathBuf::from("subs/sub3/doc1.arr"), vec![
                Fingerprint { hash: 11, lines: (3, 11) },
                Fingerprint { hash: 11, lines: (22, 24) },
                Fingerprint { hash: 77, lines: (24, 24) },
                Fingerprint { hash: 11, lines: (27, 44) },
                Fingerprint { hash: 17, lines: (50, 51) }
            ]),
            Doc::Processed(PathBuf::from("subs/sub3/doc2.arr"), vec![
                Fingerprint { hash: 88, lines: (16, 24) },
                Fingerprint { hash: 30, lines: (25, 30) },
                Fingerprint { hash: 33, lines: (55, 63) }
            ])
        ]
    };

    let m1: HashSet<i64> = [11, 17, 20].iter().cloned().collect();
    let p1 = SubPair {
        a: &a,
        a_percent: 22.331,
        b: &b,
        b_percent: 39.273,
        matches: m1,
        percentile: 71.3
    };
    let m2: HashSet<i64> = [11, 17].iter().cloned().collect();
    let p2 = SubPair {
        a: &a,
        a_percent: 36.1132,
        b: &c,
        b_percent: 78.12,
        matches: m2,
        percentile: 45.
    };
    let m3: HashSet<i64> = [11, 17, 33].iter().cloned().collect();
    let p3 = SubPair {
        a: &b,
        a_percent: 48.1,
        b: &c,
        b_percent: 96.1235,
        matches: m3,
        percentile: 14.2
    };

    let subs = vec![&a, &b, &c];
    let pairs = vec![p3, p1, p2];
    let mode = SubFileMode::Multi;
    let out_file: Option<&Path> = None; //Some(&Path::new("test-dirs/tinker/output.txt"));

    render_results(subs, pairs, &mode, out_file);
}