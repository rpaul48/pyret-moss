/* file_io.rs: File I/O */

use std::process;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use crate::fingerprint::Fingerprint;

// Sub represents a student submission.
// Depending on whether input submissions are directories or
// indiv. files, the dir_name field will be Some or None
#[derive(Debug)]
pub struct Sub<'a> {
    pub dir_name: Option<&'a Path>,
    pub documents: Vec<Doc<'a>>
}

// Doc represents a file within a submission.
// Docs are initialized as Unprocessed (contents have not yet been
// read), and become Processed once they have been fingerprinted
#[derive(Debug)]
pub enum Doc<'a> {
    Unprocessed(&'a Path),
    Processed(&'a Path, Vec<Fingerprint>)
}

// construct a vector of PathBufs to all files in a given directory 
// that pass the given predicate
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

// get paths to all .arr files in a given directory
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
            eprintln!("Error: Failed to read .arr files in `{}`", dir.display());
            process::exit(1);
        },
    }
}


#[cfg(test)]
mod file_io_tests {
    use super::*;
    use std::ffi::OsStr;

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
                mk_pathb(dir, "pyret-file.arr"),
                mk_pathb(dir, "second-pyret.arr"),
                mk_pathb(dir, "markdown.md"),
                mk_pathb(dir, "text-file.txt")
            ];

            assert_paths_in_dir(&dir, &mut expected, |p| true)?;
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
                mk_pathb(dir, "dir")
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
    fn test_arr_files_in_dir() -> io::Result<()> {
        let dir = "./test-dirs/test/read-dir-contents/";

        let mut out = arr_files_in_dir(Path::new(&dir));
        let mut exp_paths = vec![
            mk_pathb(dir, "pyret-file.arr"),
            mk_pathb(dir, "second-pyret.arr")
        ];

        out.sort();
        exp_paths.sort();

        assert_eq!(out, exp_paths);

        Ok(())
    }


}