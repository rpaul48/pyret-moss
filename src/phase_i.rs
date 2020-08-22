/* Phase I: Normalize/fingerprint all submissions */

use fnv::FnvHashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use crate::{Doc, Sub};
use crate::file_io;
use crate::fingerprint::{self, Fingerprint};
use crate::normalize;

// Read a file's contents into memory and normalize/fingerprint it
// k, t are fingerprint params
fn analyze_file(path: &Path, k: i32, t: i32) -> io::Result<Vec<Fingerprint>> {
    // read file text
    let mut file = File::open(path.to_str().unwrap())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // normalize & fingerprint
    let norm = normalize::normalize(&contents[..]);
    let fps = fingerprint::fingerprint(norm, k, t);

    Ok(fps)
}

// Construct a set of fingerprints to ignore by
// reading/normalizing/fingerprinting the given files
// k, t are fingerprint params
pub fn make_ignore_set(ignore_dir: &Path, k: i32, t: i32) -> HashSet<i64> {
    let ignore_paths = file_io::arr_files_in_dir(ignore_dir);
    let mut ignore_set = HashSet::new();

    if ignore_paths.len() == 0 {
        err!("no .arr files to ignore in `{}`", ignore_dir.display());
    }

    for path in ignore_paths.iter() {
        // normalize/fingerprint this ignore file
        let fps = match analyze_file(path, k, t) {
            Ok(v) => v,
            Err(e) => { err!("failed to analyze file {}: {}", path.display(), e); }
        };

        // add all fingerprint hashes to ignore set
        for fp in fps.iter() { ignore_set.insert(fp.hash); }
    }

    ignore_set
}

// Read/normalize/fingerprint documents in given submissions, constructing
// a hashmap from fingerprint hashes to the set of subs that share that hash
pub fn analyze_subs<'a>(subs: &'a mut Vec<&'a mut Sub>, ignore: Option<HashSet<i64>>,
    k: i32, t: i32, verbose: bool) -> FnvHashMap<i64, HashSet<&'a Sub>> {
    if verbose { println!("\nAnalyzing all submission content..."); }

    let mut fp_to_subs = FnvHashMap::default();

    // for each submission
    for sub in subs.iter_mut() {
        if verbose {
            if let Some(path) = &sub.dir_name {
                println!("\tprocessing {}", path.display());
            }
        }

        let mut sub_fps = HashSet::new();

        // for each document in this submission
        for doc in sub.documents.iter_mut() {
            let doc_path = match doc {
                Doc::Unprocessed(p) => p,
                Doc::Processed(_, _) => panic!("Already processed document: {:?}", doc),
            };

            // attempt to normalize/fingerprint document
            let fps = match analyze_file(doc_path, k, t) {
                Ok(v) => v,
                Err(e) => { err!("failed to analyze file {}: {}", doc_path.display(), e); }
            };

            let orig_amount_fps = fps.len();    // store original # fingerprints before ignore

            // filter out ignored fingerprints, if any
            let fps = match ignore {
                Some(ref set) => {
                    fps.into_iter()
                        .filter(|fp| !set.contains(&fp.hash))
                        .collect::<Vec<Fingerprint>>()
                },
                None => fps,
            };

            if verbose { 
                let fp_count = fps.len();
                println!("\t\tanalyzing {}: {} fingerprints ({} ignored)", 
                    doc_path.file_name().unwrap().to_str().unwrap(),
                    fp_count,
                    orig_amount_fps - fp_count);
            }

            // add included fingerprints for this doc to the set for this submission
            for fp in fps.iter() { sub_fps.insert(fp.clone()); }

            // update Doc at this position to include fingerprints
            *doc = Doc::Processed(doc_path.to_path_buf(), fps);
        }

        for fp in sub_fps.iter() {
            // add this sub to the vec of subs that share this fingerprint
            fp_to_subs.entry(fp.hash)
                .or_insert_with(HashSet::new)
                .insert(&**sub);  // sub is &mut&mut Sub, so this converts to &Sub
        }
    }

    fp_to_subs
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_analyze_file() -> io::Result<()> {
        let dir = "./test-dirs/test/read-and-fingerprint/";

        {
            let exp_fps = vec![
                Fingerprint { hash: 1718972022, lines: (2, 2) }, 
                Fingerprint { hash: 1853237366, lines: (2, 2) }, 
                Fingerprint { hash: 678832442, lines: (2, 2) }, 
                Fingerprint { hash: 691697194, lines: (2, 3) }, 
                Fingerprint { hash: 712402286, lines: (3, 4) }];

            // k=4, t=6
            let out_fps = analyze_file(
                &Path::new(&format!("{}{}", dir, "a.arr")), 4, 6)?;

            assert_eq!(exp_fps, out_fps);
        }
        {
            let exp_fps = vec![
                Fingerprint { hash: 1684351798, lines: (1, 3) }, 
                Fingerprint { hash: 711221822, lines: (1, 3) }, 
                Fingerprint { hash: 981235476, lines: (3, 4) }, 
                Fingerprint { hash: 678832721, lines: (4, 5) }];

            // k=5, t=10
            let out_fps = analyze_file(
                &Path::new(&format!("{}{}", dir, "b.arr")), 5, 10)?;

            assert_eq!(exp_fps, out_fps);
        }

        Ok(())
    }

    #[test]
    fn test_ignore_set() {
        {
            let ignore = make_ignore_set(&Path::new("test-dirs/test/ignore"), 10, 25);

            let exp_set: HashSet<i64> = [
                // ignore1.arr
                390399223, 440994808, 317504781, 691281357, 754436089, 666964489, 317504781, 155049740,

                // ignore2.arr
                743504368, 743504367, 827075361, 743487652, 317508614, 834200077, 123232836, 683703066, 
                793965034, 760996000, 666965465, 317508627, 255995164, 123232842, 17122932, 16820579,

                // ignore3.arr
                560266828
            ].iter().cloned().collect();

            assert_eq!(ignore, exp_set);
        }
        {
            let ignore = make_ignore_set(&Path::new("test-dirs/test/ignore"), 6, 10);

            let exp_set: HashSet<i64> = [
                // ignore1.arr
                1684412226, 712063801, 1768244249, 1684412998, 762659386, 1702108489, 711604553, 1668106304, 
                1684426514, 673394523, 577067955, 1667576388, 761543728, 778189628, 573197906, 694833831, 
                1668106304, 1684426514, 673394523, 577067955, 1667576388, 761543737, 1668299207, 1835939404, 778195278,

                // ignore2.arr
                1869827926, 1635016533, 1668106304, 1684426579, 1668106304, 1684426579, 1702319157, 1768831835, 
                1836145732, 1869827926, 1635016533, 1668106304, 1684426514, 673398356, 578049201, 1634946100, 
                761738292, 778193724, 573376343, 578005182, 1684451665, 1685340991, 1718305856, 1785479480, 
                1685341505, 1685339454, 1685336822, 573198394, 694827687, 1668106304, 1684426514, 673398369, 
                578052544, 1869829390, 778196826, 573376343, 578005188, 1684843345, 1684474119, 842268931, 
                859374488, 879027156, 1685312720, 1684486423, 825818887,

                // ignore3.arr
                981285224, 678896994
            ].iter().cloned().collect();

            assert_eq!(ignore, exp_set);
        }
    }

    // Converts an array of Sub references into a hashset of the same refs
    fn set<'a>(array: &[&'a Sub]) -> HashSet<&'a Sub> {
        array.iter().cloned().collect()
    }

    #[test]
    fn test_analyze_single_files() {
        // original submissions
        let mut sub1 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file/sub1.arr"))
            ]
        };
        let mut sub2 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file/sub2.arr"))
            ]
        };

        let mut submissions = vec![&mut sub1, &mut sub2];
        let out = analyze_subs(&mut submissions, None, 10, 60, false);

        // submissions after analysis
        let proc_sub1 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Processed(
                    PathBuf::from("test-dirs/test/single-file/sub1.arr"),
                    vec![
                        Fingerprint { hash: 5421077, lines: (11, 12) }, 
                        Fingerprint { hash: 31722361, lines: (15, 16) }, 
                        Fingerprint { hash: 30182096, lines: (16, 16) },
                        Fingerprint { hash: 14933625, lines: (17, 18) }, 
                        Fingerprint { hash: 73943364, lines: (19, 19) }
                    ])
            ]
        };

        let proc_sub2 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Processed(
                    PathBuf::from("test-dirs/test/single-file/sub2.arr"),
                    vec![
                        Fingerprint { hash: 5421077, lines: (8, 10) }, 
                        Fingerprint { hash: 14933625, lines: (13, 14) }, 
                        Fingerprint { hash: 73943364, lines: (15, 15) }
                    ])
            ]
        };

        let mut exp_out: FnvHashMap<i64, HashSet<&Sub>> = FnvHashMap::default();

        exp_out.insert(5421077,  set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(31722361, set(&[&proc_sub1]));
        exp_out.insert(30182096, set(&[&proc_sub1]));
        exp_out.insert(14933625, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(73943364, set(&[&proc_sub1, &proc_sub2]));

        assert_eq!(out, exp_out);
    }

    #[test]
    fn test_analyze_multi_files() {
        // original submissions
        let mut sub1 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub1")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub1/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub1/main.arr"))
            ]
        };
        let mut sub2 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub2")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub2/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub2/main.arr"))
            ]
        };

        let mut submissions = vec![&mut sub1, &mut sub2];
        let out = analyze_subs(&mut submissions, None, 5, 15, false);

        // submissions after analysis
        let proc_sub1 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub1")),
            documents: vec![
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub1/common.arr"),
                    vec![
                        Fingerprint { hash: 712012601, lines: (1, 2) }, 
                        Fingerprint { hash: 762608186, lines: (2, 2) },
                        Fingerprint { hash: 711221850, lines: (2, 5) }, 
                        Fingerprint { hash: 678833506, lines: (7, 7) }
                    ]),
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub1/main.arr"),
                    vec![
                        Fingerprint { hash: 711358008, lines: (1, 3) }, 
                        Fingerprint { hash: 678832678, lines: (3, 3) }, 
                        Fingerprint { hash: 691697430, lines: (3, 5) }, 
                        Fingerprint { hash: 712407124, lines: (5, 6) }, 
                        Fingerprint { hash: 674572885, lines: (7, 7) }, 
                        Fingerprint { hash: 674703957, lines: (8, 8) }, 
                        Fingerprint { hash: 674050581, lines: (9, 9) }
                    ])
            ]
        };
        let proc_sub2 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub2")),
            documents: vec![
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub2/common.arr"),
                    vec![
                        Fingerprint { hash: 711221822, lines: (1, 3) }, 
                        Fingerprint { hash: 678833506, lines: (8, 8) }
                    ]),
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub2/main.arr"),
                    vec![
                        Fingerprint { hash: 711358008, lines: (1, 4) }, 
                        Fingerprint { hash: 678832678, lines: (4, 4) }, 
                        Fingerprint { hash: 691697430, lines: (4, 6) }, 
                        Fingerprint { hash: 712402522, lines: (6, 7) }, 
                        Fingerprint { hash: 980822283, lines: (9, 10) }, 
                        Fingerprint { hash: 674572885, lines: (10, 10) }, 
                        Fingerprint { hash: 674703957, lines: (11, 11) }, 
                        Fingerprint { hash: 674050581, lines: (12, 12) }, 
                        Fingerprint { hash: 674376277, lines: (13, 13) }
                    ])
            ]
        };

        let mut exp_out: FnvHashMap<i64, HashSet<&Sub>> = FnvHashMap::default();
        exp_out.insert(712012601, set(&[&proc_sub1]));
        exp_out.insert(762608186, set(&[&proc_sub1]));
        exp_out.insert(711221850, set(&[&proc_sub1]));
        exp_out.insert(712407124, set(&[&proc_sub1]));
        exp_out.insert(678833506, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(711358008, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(678832678, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(691697430, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(674572885, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(674703957, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(674050581, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(711221822, set(&[&proc_sub2]));
        exp_out.insert(712402522, set(&[&proc_sub2]));
        exp_out.insert(980822283, set(&[&proc_sub2]));
        exp_out.insert(674376277, set(&[&proc_sub2]));

        assert_eq!(out, exp_out);
    }

    #[test]
    fn test_analyze_with_ignore() {
        // original submissions
        let mut sub1 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub1")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub1/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub1/main.arr"))
            ]
        };
        let mut sub2 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub2")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub2/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub2/main.arr"))
            ]
        };

        // use an ignore set
        let ignore_set: HashSet<i64> = [
            711221850,
            711358008,
            678832678,
            691697430,
            711221822,
            980822283
        ].iter().cloned().collect();

        let mut submissions = vec![&mut sub1, &mut sub2];
        let out = analyze_subs(&mut submissions, Some(ignore_set), 5, 15, false);

        // submissions after analysis
        let proc_sub1 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub1")),
            documents: vec![
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub1/common.arr"),
                    vec![
                        Fingerprint { hash: 712012601, lines: (1, 2) }, 
                        Fingerprint { hash: 762608186, lines: (2, 2) },
                        Fingerprint { hash: 678833506, lines: (7, 7) }
                    ]),
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub1/main.arr"),
                    vec![
                        Fingerprint { hash: 712407124, lines: (5, 6) }, 
                        Fingerprint { hash: 674572885, lines: (7, 7) }, 
                        Fingerprint { hash: 674703957, lines: (8, 8) }, 
                        Fingerprint { hash: 674050581, lines: (9, 9) }
                    ])
            ]
        };
        let proc_sub2 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub2")),
            documents: vec![
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub2/common.arr"),
                    vec![
                        Fingerprint { hash: 678833506, lines: (8, 8) }
                    ]),
                Doc::Processed(
                    PathBuf::from("test-dirs/test/multi-file/sub2/main.arr"),
                    vec![
                        Fingerprint { hash: 712402522, lines: (6, 7) }, 
                        Fingerprint { hash: 674572885, lines: (10, 10) }, 
                        Fingerprint { hash: 674703957, lines: (11, 11) }, 
                        Fingerprint { hash: 674050581, lines: (12, 12) }, 
                        Fingerprint { hash: 674376277, lines: (13, 13) }
                    ])
            ]
        };

        let mut exp_out: FnvHashMap<i64, HashSet<&Sub>> = FnvHashMap::default();
        exp_out.insert(712012601, set(&[&proc_sub1]));
        exp_out.insert(762608186, set(&[&proc_sub1]));
        exp_out.insert(712407124, set(&[&proc_sub1]));
        exp_out.insert(678833506, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(674572885, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(674703957, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(674050581, set(&[&proc_sub1, &proc_sub2]));
        exp_out.insert(712402522, set(&[&proc_sub2]));
        exp_out.insert(674376277, set(&[&proc_sub2]));

        assert_eq!(out, exp_out);
    }

}