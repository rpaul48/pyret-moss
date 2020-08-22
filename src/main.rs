#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prettytable;
#[macro_use] mod error;
extern crate regex;
use std::path::PathBuf;
use std::collections::HashSet;
use crate::fingerprint::Fingerprint;
mod cli;
mod fingerprint;
mod normalize;
mod file_io;
mod io_redirect;
mod phase_i;
mod phase_ii;
mod phase_iii;
mod results;

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
    // parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let (sub_dir, opts) = cli::parse_args(&args);

    // get set of filenames to ignore (or empty if none)
    let ignore_files = match opts.ignore_files {
        Some(s) => s,
        None => HashSet::new()
    };

    if opts.verbose {
        if ignore_files.len() > 0 {
            println!("Ignoring files: {:?}", ignore_files);
        }
    }

    // if a directory of files to ignore is given, construct a set 
    // of fingerprints to ignore when considering matches
    let ignore_set = match opts.ignore_content_dir {
        Some(p) => {
            if opts.verbose {
                println!("Ignoring content from {}", p.display());
            }

            Some(phase_i::make_ignore_set(p, opts.k, opts.t))
        },
        None => None,
    };

    // construct structs representing each submission in the indicated 
    // directory & submission mode (single/multi file)
    let mut subs = file_io::construct_subs(sub_dir, &opts.sub_mode, &ignore_files, opts.verbose);

    // construct vec of mutable borrows of each sub for passing to sub analysis
    let mut mut_sub_refs = Vec::new();
    for sub in subs.iter_mut() {
        mut_sub_refs.push(sub);
    }

    // process all documents in each submission, mapping fingerprints 
    // to all submissions in which they appeared
    let hash_to_subs = phase_i::analyze_subs(&mut mut_sub_refs, ignore_set, opts.k, opts.t, opts.verbose);

    // group submissions into pairs based on shared fingerprints, and 
    // order according to the number of fingerprints shared
    let (sub_pairs, total_pairs) = phase_ii::find_overlaps(&hash_to_subs, opts.match_threshold, opts.verbose);

    // render a report to the user detailing submission overlap
    results::render_results(sub_pairs, &opts.sub_mode, opts.out_file, 
        opts.match_threshold, total_pairs, opts.no_pauses, opts.verbose);
}