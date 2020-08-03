/* Phase I: Normalize/fingerprint all submissions */

use fnv::FnvHashMap;
use std::collections::HashSet;
use crate::error::err;
use crate::file_io::{self, Sub, Doc};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::fingerprint;
use crate::normalize;

// Construct a set of fingerprints to ignore by 
// reading/normalizing/fingerprinting the given files 
pub fn make_ignore_set(ignore_dir: &Path) -> io::Result<HashSet<i64>> {
    let ignore_paths = file_io::arr_files_in_dir(ignore_dir);
    let ignore_set = HashSet::new();

    for path in ignore_paths.iter() {
        println!("Ignoring: {:?}", path.display());

        let mut file = File::open(path.to_str().unwrap())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        println!("File contents: `{}`", contents);

        let norm = normalize::normalize(&contents[..]);
        let fps = fingerprint::fingerprint(norm);

        for fp in fps.iter() {
            ignore_set.insert(fp.hash);
        }
    }

    Ok(ignore_set)
}

// Read/normalize/fingerprint documents in given submissions, constructing
// a hashmap from fingerprint hashes to the set of subs that share that hash
pub fn analyze_subs(subs: Vec<Sub>, ignore: Option<HashSet<i64>>) 
    -> FnvHashMap<i64, Vec<&Sub>> {
    unimplemented!();
    /*
    fingerprints_to_subs = FnvHashMap
    for each Sub:
        submission_fingerprints = HashSet
        for each Doc in this sub:
            Normalize document
            Fingerprint document
            Eliminate any fingerprints that are in the ignore set (if given)
            Add fingerprints for this document to submission_fingerprints
        For each print in the submission_fingerprints
            Add this submission to the set of submissions mapped to by this fingerprint
    return fingerprints_to_subs
    */
}