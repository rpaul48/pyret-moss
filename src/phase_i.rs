/* Phase I: Normalize/fingerprint all submissions */

use fnv::FnvHashMap;
use std::collections::HashSet;
use crate::{Doc, Sub};
use crate::file_io;
use crate::error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::fingerprint::{self, Fingerprint};
use crate::normalize;

// Read a file's contents into memory & normalize/fingerprint it
// k, t are fingerprint params
fn read_and_fingerprint(path: &Path, k: i32, t: i32) -> io::Result<Vec<Fingerprint>> {
    // read file text
    let mut file = File::open(path.to_str().unwrap())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // normalize & fingerprint
    let norm = normalize::normalize(&contents[..]);
    Ok(fingerprint::fingerprint(norm, k, t))
}

// Construct a set of fingerprints to ignore by
// reading/normalizing/fingerprinting the given files
// k, t are fingerprint params
pub fn make_ignore_set(ignore_dir: &Path, k: i32, t: i32) -> io::Result<HashSet<i64>> {
    let ignore_paths = file_io::arr_files_in_dir(ignore_dir);
    let mut ignore_set = HashSet::new();

    if ignore_paths.len() == 0 {
        error::err(&format!("no .arr files to ignore in `{}`", ignore_dir.display()));
    }

    for path in ignore_paths.iter() {
        let fps = read_and_fingerprint(path, k, t)?;

        // add all fingerprint hashes to ignore set
        for fp in fps.iter() { ignore_set.insert(fp.hash); }
    }

    Ok(ignore_set)
}

// Read/normalize/fingerprint documents in given submissions, constructing
// a hashmap from fingerprint hashes to the set of subs that share that hash
pub fn analyze_subs<'a>(subs: &'a mut Vec<&'a mut Sub>, ignore: Option<HashSet<i64>>,
    k: i32, t: i32) -> io::Result<FnvHashMap<i64, Vec<&'a Sub>>> {

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

            let fps = read_and_fingerprint(doc_path, k, t)?;

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
            *doc = Doc::Processed(doc_path.to_path_buf(), fps);
        }

        for fp in sub_fps.iter() {
            // add this sub to the vec of subs that share this fingerprint
            fp_to_subs.entry(fp.hash)
                .or_insert_with(Vec::new)
                .push(&**sub);  // sub is &mut&mut Sub, so this converts to &Sub
        }
    }

    Ok(fp_to_subs)
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_read_and_fingerprint() -> io::Result<()> {
    //     let dir = "./test-dirs/test/read-and-fingerprint/";

    //     // they can't both succeed at the moment

    //     {
    //         let exp_fps = vec![
    //             Fingerprint { hash: 3154647, lines: (2, 2) }, 
    //             Fingerprint { hash: 3391766, lines: (2, 2) }, 
    //             Fingerprint { hash: 1306367, lines: (2, 2) }, 
    //             Fingerprint { hash: 1280869, lines: (2, 3) }, 
    //             Fingerprint { hash: 1367861, lines: (3, 4) }];

    //         // k=4, t=6
    //         let out = read_and_fingerprint(&Path::new(&format!("{}{}", dir, "a.arr")))?;

    //         assert_eq!(exp_fps, out);
    //     }
    //     {
    //         let exp_fps = vec![
    //             Fingerprint { hash: 95404550, lines: (1, 3) }, 
    //             Fingerprint { hash: 94626066, lines: (1, 3) }, 
    //             Fingerprint { hash: 41863892, lines: (1, 3) }, 
    //             Fingerprint { hash: 57373058, lines: (3, 4) }, 
    //             Fingerprint { hash: 40498820, lines: (4, 5) }];

    //         // k=5, t=10
    //         let out = read_and_fingerprint(&Path::new(&format!("{}{}", dir, "b.arr")))?;

    //         assert_eq!(exp_fps, out);
    //     }

    //     Ok(())
    // }

}