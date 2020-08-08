#[macro_use] extern crate lazy_static;
extern crate regex;
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
// read), and become Processed once they have been fingerprinted
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Doc {
    Unprocessed(PathBuf),
    Processed(PathBuf, Vec<Fingerprint>)
}

fn main() {
    
}