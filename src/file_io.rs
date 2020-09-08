/// file_io.rs: File I/O

use std::fs;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::io;
use crate::{Sub, Doc};
use crate::cli::SubFileMode;

/// Construct a vector of PathBufs to all files in a given
/// directory that pass the given predicate
fn paths_in_dir<F>(dir: &Path, keep: F) -> io::Result<Vec<PathBuf>>
    where F: Fn(&PathBuf) -> bool {
    let mut paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if keep(&path) {
            paths.push(path);
        }
    }

    Ok(paths)
}

/// Gets paths to all dirs in a given directory
fn dirs_in_dir(dir: &Path) -> Vec<PathBuf> {
    match paths_in_dir(dir, |p| p.is_dir()) {
        Ok(paths) => paths,
        Err(e) => {
            err!("failed to read dirs in `{}`: {}", dir.display(), e);
        },
    }
}

/// Gets paths to all .arr files in a given directory
pub fn arr_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let is_arr = |p: &PathBuf| {
        match p.extension() {
            Some(ext) => ext == "arr",
            None => false,
        }
    };
    match paths_in_dir(dir, is_arr) {
        Ok(paths) => paths,
        Err(e) => {
            err!("failed to read .arr files in `{}`: {}", dir.display(), e);
        },
    }
}

/// Build a vector of submissions by traversing the given directory
/// in a manner specified by the sub_mode
pub fn construct_subs(sub_dir: &Path, sub_mode: &SubFileMode,
    ignore_files: &HashSet<String>, verbose: bool) -> Vec<Sub> {
    let mut subs = Vec::new();

    if !sub_dir.is_dir() {  // validate submission directory
        err!("submission directory `{}` is not a dir", sub_dir.display());
    }

    if verbose {
        match sub_mode {
            SubFileMode::Single => {
                println!("\nSubmission mode: single .arr files");
            },
            SubFileMode::Multi => {
                println!("\nSubmission mode: subdirectories of .arr files");
            }
        };
        println!("Entering submissions directory... ({})", sub_dir.display());
    }

    match sub_mode {
        // treat submissions as individual .arr files within the sub_dir
        SubFileMode::Single => {
            let sub_files = arr_files_in_dir(sub_dir);

            if sub_files.len() == 0 {
                err!("submission directory `{}` contains no .arr files (omit -s for multi-file mode)", sub_dir.display());
            }

            // for each submission (.arr file)
            for file in sub_files.iter() {
                if verbose { println!("\tcreating submission {}", file.display()); }

                let doc = Doc::Unprocessed(file.to_path_buf());

                subs.push(Sub {
                    dir_name: None,
                    documents: vec![doc]
                });
            }
        },
        // treat submissions as dirs of .arr files within sub_dir
        SubFileMode::Multi => {
            let sub_dirs = dirs_in_dir(sub_dir);

            if sub_dirs.len() == 0 {
                err!("submission directory `{}` contains no subdirectories (use -s for single-file mode)", sub_dir.display());
            }

            // for each submission (subdirectory)
            for sub in sub_dirs.iter() {
                if verbose { println!("\tcreating submission {}", sub.display()); }

                // read files for this submission
                let files = arr_files_in_dir(sub.as_path());
                let mut docs = Vec::new();

                // add an unprocessed document for each file in the submission
                for file in files.iter() {
                    let fname = file.file_name().unwrap().to_str().unwrap();

                    // don't include files that are ignored (by filename)
                    if !ignore_files.contains(fname) {
                        if verbose { println!("\t\tadding document {}", fname); }

                        docs.push(Doc::Unprocessed(file.to_path_buf()));
                    }
                }

                subs.push(Sub {
                    dir_name: Some(sub.to_path_buf()),
                    documents: docs
                });
            }
        },
    };

    subs    // return constructed submissions
}


#[cfg(test)]
mod tests {
    use super::*;

    // construct a PathBuf from a dir path & a file within it
    fn mk_pathb(d: &str, f: &str) -> PathBuf {
        PathBuf::from(format!("{}{}", d, f))
    }

    #[test]
    fn test_paths_in_dir() -> io::Result<()> {
        let dir = "./test-dirs/test/read-dir-contents/";

        // test paths_in_dir with a given predicate & expected output (ordering insensitive)
        fn assert_paths_in_dir<F>(dir: &str, exp_paths: &mut Vec<PathBuf>, predicate: F)
            -> io::Result<()> where F: Fn(&PathBuf) -> bool {
            let mut paths = paths_in_dir(Path::new(dir), predicate)?;

            paths.sort();
            exp_paths.sort();

            assert_eq!(paths, *exp_paths);
            Ok(())
        }

        {
            // accept all paths in directory
            let mut expected = vec![
                mk_pathb(dir, "dir"),
                mk_pathb(dir, "dir2"),
                mk_pathb(dir, "pyret-file.arr"),
                mk_pathb(dir, "second-pyret.arr"),
                mk_pathb(dir, "markdown.md"),
                mk_pathb(dir, "text-file.txt")
            ];

            assert_paths_in_dir(&dir, &mut expected, |_| true)?;
        }
        {
            // accept .txt files only
            let mut expected = vec![
                mk_pathb(dir, "text-file.txt")
            ];

            assert_paths_in_dir(&dir, &mut expected, |p| {
                match p.extension() {
                    Some(ext) => ext == "txt",
                    None => false,
                }
            })?;
        }
        {
            // accept .arr files & dirs
            let mut expected = vec![
                mk_pathb(dir, "pyret-file.arr"),
                mk_pathb(dir, "second-pyret.arr"),
                mk_pathb(dir, "dir"),
                mk_pathb(dir, "dir2")
            ];

            assert_paths_in_dir(&dir, &mut expected, |p| {
                match p.extension() {
                    Some(ext) => ext == "arr",
                    None => p.is_dir(),
                }
            })?;
        }

        Ok(())
    }

    #[test]
    fn test_dirs_in_dir() {
        let dir = "./test-dirs/test/read-dir-contents/";

        let mut out = dirs_in_dir(Path::new(&dir));
        let mut exp_paths = vec![
            mk_pathb(dir, "dir"),
            mk_pathb(dir, "dir2")
        ];

        out.sort();
        exp_paths.sort();

        assert_eq!(out, exp_paths);
    }

    #[test]
    fn test_arr_files_in_dir() {
        let dir = "./test-dirs/test/read-dir-contents/";

        let mut out = arr_files_in_dir(Path::new(&dir));
        let mut exp_paths = vec![
            mk_pathb(dir, "pyret-file.arr"),
            mk_pathb(dir, "second-pyret.arr")
        ];

        out.sort();
        exp_paths.sort();

        assert_eq!(out, exp_paths);
    }

    #[test]
    fn test_construct_subs() {
        // construct a submission from optional dirname & paths of docs
        fn mk_sub(dir_name: Option<&str>, docs: Vec<&str>) -> Sub {
            Sub {
                dir_name: match dir_name {
                    Some(name) => Some(PathBuf::from(name)),
                    None => None,
                },
                documents: docs.iter().map(|s| {
                    Doc::Unprocessed(PathBuf::from(s))
                }).collect()
            }
        }

        // single-file subs
        {
            let sub_dir = Path::new("./test-dirs/test/single-file");
            let mut out = construct_subs(sub_dir, &SubFileMode::Single, &HashSet::new(), false);
            let mut exp_subs = vec![
                mk_sub(None, vec![
                    "./test-dirs/test/single-file/sub1.arr"
                ]),
                mk_sub(None, vec![
                    "./test-dirs/test/single-file/sub2.arr"
                ])
            ];

            out.sort(); exp_subs.sort();
            assert_eq!(out, exp_subs);
        }
        // multi-file subs
        {
            let sub_dir = Path::new("./test-dirs/test/multi-file");
            let mut out = construct_subs(sub_dir, &SubFileMode::Multi, &HashSet::new(), false);
            let mut exp_subs = vec![
                mk_sub(Some("./test-dirs/test/multi-file/sub1"), vec![
                    "./test-dirs/test/multi-file/sub1/common.arr",
                    "./test-dirs/test/multi-file/sub1/main.arr"
                ]),
                mk_sub(Some("./test-dirs/test/multi-file/sub2"), vec![
                    "./test-dirs/test/multi-file/sub2/common.arr",
                    "./test-dirs/test/multi-file/sub2/main.arr"
                ])
            ];

            for o in out.iter_mut() { o.documents.sort(); }
            for e in exp_subs.iter_mut() { e.documents.sort(); }
            out.sort(); exp_subs.sort();

            assert_eq!(out, exp_subs);
        }
        // check ignore files by name
        {
            let sub_dir = Path::new("./test-dirs/test/multi-file");

            // ignore common.arr files
            let mut ignore_files = HashSet::new();
            ignore_files.insert(String::from("common.arr"));

            let mut out = construct_subs(sub_dir, &SubFileMode::Multi, &ignore_files, false);
            let mut exp_subs = vec![
                mk_sub(Some("./test-dirs/test/multi-file/sub1"), vec![
                    "./test-dirs/test/multi-file/sub1/main.arr"
                ]),
                mk_sub(Some("./test-dirs/test/multi-file/sub2"), vec![
                    "./test-dirs/test/multi-file/sub2/main.arr"
                ])
            ];

            for o in out.iter_mut() { o.documents.sort(); }
            for e in exp_subs.iter_mut() { e.documents.sort(); }
            out.sort(); exp_subs.sort();

            assert_eq!(out, exp_subs);
        }
    }
}
