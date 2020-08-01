/* Phase I: Normalize/fingerprint all submissions */

use fnv::FnvHashMap;
use std::collections::HashSet;
use crate::main::Sub;
use crate::main::Doc;

// Construct a set of fingerprints to ignore by 
// reading/normalizing/fingerprinting the given files 
pub fn make_ignore_set(files: Vec<String>) -> HashSet<i64> {
    unimplemented!();
}

// Read/normalize/fingerprint documents in given submissions, constructing
// a hashmap from fingerprint hashes to the set of subs that share that hash
pub fn analyze_subs(subs: Vec<Sub>, ignore: Option<HashSet<i64>>) -> FnvHashMap {
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