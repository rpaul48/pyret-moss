/* Phase I: Normalize/fingerprint all submissions */

use fnv::FnvHashMap;
use std::collections::HashSet;
use crate::file_io::{self, Sub, Doc};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use crate::fingerprint::{self, Fingerprint};
use crate::normalize;

fn read_and_fingerprint(path: &Path) -> io::Result<Vec<Fingerprint>> {
    // read file text
    let mut file = File::open(path.to_str().unwrap())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // normalize & fingerprint
    let norm = normalize::normalize(&contents[..]);
    Ok(fingerprint::fingerprint(norm))
}

// Construct a set of fingerprints to ignore by 
// reading/normalizing/fingerprinting the given files 
pub fn make_ignore_set(ignore_dir: &Path) -> io::Result<HashSet<i64>> {
    let ignore_paths = file_io::arr_files_in_dir(ignore_dir);
    let mut ignore_set = HashSet::new();

    for path in ignore_paths.iter() {
        let fps = read_and_fingerprint(path)?;

        // add all fingerprint hashes to ignore set
        for fp in fps.iter() { ignore_set.insert(fp.hash); }
    }

    Ok(ignore_set)
}

// Read/normalize/fingerprint documents in given submissions, constructing
// a hashmap from fingerprint hashes to the set of subs that share that hash
pub fn analyze_subs<'a>(subs: &'a mut Vec<&'a mut Sub>, ignore: Option<HashSet<i64>>) 
    -> io::Result<FnvHashMap<i64, Vec<&'a Sub<'a>>>> {

    let mut fp_to_subs = FnvHashMap::default();

    // for each submission
    for sub in subs.iter_mut() {
        let mut sub_fps = HashSet::new();

        // for each document in this submission
        for doc in sub.documents.iter_mut() {
            let doc_path = match doc {
                Doc::Unprocessed(p) => p,
                Doc::Processed(_, _) => panic!("Already processed document: {:?}", doc),
            };

            let fps = read_and_fingerprint(doc_path)?;

            // filter out ignored fingerprints, if any
            let fps = match ignore {
                Some(ref set) => {
                    fps.into_iter()
                        .filter(|fp| !set.contains(&fp.hash))
                        .collect::<Vec<Fingerprint>>()
                },
                None => fps,
            };

            // add included fingerprints for this doc to the set for this submission
            for fp in fps.iter() { sub_fps.insert(fp.clone()); }

            // update Doc at this position to include fingerprints
            *doc = Doc::Processed(doc_path, fps);
        }

        for fp in sub_fps.iter() {
            // add this sub to the vec of subs that share this fingerprint
            fp_to_subs.entry(fp.hash)
                .or_insert_with(Vec::new)
                .push(&**sub);
        }
    }

    Ok(fp_to_subs)
}