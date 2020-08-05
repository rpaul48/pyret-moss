/* cli.rs: Functions for providing the command-line interface */

use std::path::Path;

/*
Mandatory args:
    - sub_dir:          path of directory where submissions can be found

Optional args:
	- sub_mode:         indicates whether subs are files or dirs
	- k:                noise threshold
    - t:                guarantee threshold
	- ignore_dir:       ignore boilerplate code (indicate a dir)
    - max_pairs_out:    limit max number of pairs of subs to report on in output
    - out_file:         where the program's result summary will be written (default stdout)
    - verbose:          more logging
*/
#[derive(Debug)]
pub struct OptArgs<'a> {
    pub sub_mode: SubFileMode,
    pub k: i32,
    pub t: i32,
    pub ignore_dir: Option<&'a Path>,
    pub max_pairs_out: Option<i32>,
    pub out_file: Option<&'a Path>,
    pub verbose: bool
}

// SubFileMode indicates how submissions should be found within 
// the directory argument the program is given:
//  1) Single assumes submissions are individual .arr files, and
//     will construct one Sub for each .arr file in the given dir
//  2) Multi assumes submissions are directories with multiple .arr files
//     within them, and will construct one Sub for each dir in the given dir.
#[derive(Debug)]
pub enum SubFileMode {
    Single,
    Multi
}

// default values of all system parameters
impl Default for OptArgs<'_> {
    fn default() -> Self {
        OptArgs {
            sub_mode: SubFileMode::Multi,
            k: 15,
            t: 20,
            ignore_dir: None,
            max_pairs_out: None,
            out_file: None,
            verbose: false
        }
    }
}

// Copy-detection for Pyret

// USAGE:
//      ./pyret-moss <SUBMISSIONS-DIR> [OPTIONS]

// SUBMISSIONS-DIR indicates a directory containing submissions (either individual
// .arr files or subdirectories of .arr files)

// OPTIONS:
//      --help                  Prints this help information
//      --single-file-subs      Indicates that submissions will be treated as single .arr files
//      -k <VALUE>              Sets the noise threshold, k
//      -t <VALUE>              Sets the guarantee threshold, t
//      --ignore <DIR>          Indicates submission matches with the .arr files in DIR should be ignored
//      --max-out <VALUE>       Limits the number of submission pairs in the output analysis to VALUE
//      -o <FILE>               Writes the analysis to FILE instead of stdout
//      --verbose               More logging