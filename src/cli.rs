/* cli.rs: Functions for providing the command-line interface */

use std::path::Path;
use crate::error;

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

// Parse command line arguments and return a Path to the 
// submissions dir, and a struct with optional arg values.
// If the help flag is included, print_help() will be called
// and the program will exit.
pub fn parse_args(args: &Vec<String>) -> (&Path, OptArgs) {
    let argc = args.len();

    // handle invalid arity
    if argc == 0 {
        panic!("Program received 0 arguments, somehow");
    } else if argc == 1 {
        error::err(&format!(
            "usage: {} [options] <submission-dir>. See --help for more.", &args[0]));
    }

    let mut options = OptArgs::default();   // start with default options
    let mut sub_dir: Option<&Path> = None;

    // unwrap the next argument or give a specific error if none available
    fn unwrap_next<'a>(flag: &str, next: Option<&'a String>) -> &'a String {
        if let Some(arg) = next {
            arg
        } else {
            error::err(&format!("expected an argument for {}", flag));
            panic!();   // won't get here
        }
    }

    let mut iter = args.iter().skip(1); // skip first arg (path to program)

    while let Some(arg) = iter.next() {
        let arg = arg.as_str();
        match arg {
            "--help" => print_help(),
            "--single-file-subs" => options.sub_mode = SubFileMode::Single,
            "-k" => {
                let k_str = unwrap_next(arg, iter.next());

                if let Ok(k) = k_str.parse::<i32>() {
                    options.k = k;
                } else {
                    error::err(&format!("invalid value for k: `{}`", k_str));
                }
            },
            "-t" => {
                let t_str = unwrap_next(arg, iter.next());

                if let Ok(t) = t_str.parse::<i32>() {
                    options.t = t;
                } else {
                    error::err(&format!("invalid value for t: `{}`", t_str));
                }
            },
            "-o" => {
                let out_file = unwrap_next(arg, iter.next());
                options.out_file = Some(&Path::new(out_file));
            },
            "--ignore" => {
                let ignore_dir = unwrap_next(arg, iter.next());
                options.ignore_dir = Some(&Path::new(ignore_dir));
            },
            "--max-out" => {
                let max_str = unwrap_next(arg, iter.next());

                if let Ok(max_pairs_out) = max_str.parse::<i32>() {
                    options.max_pairs_out = Some(max_pairs_out);
                } else {
                    error::err(&format!("invalid value for --max-out: `{}`", max_str));
                }
            },
            "--verbose" => options.verbose = true,
            _ => {
                // check for unrecognized flags
                if arg.starts_with('-') {
                    error::err(&format!("unrecognized flag `{}`", arg));
                } else if let None = sub_dir {
                    // assume this is the submissions directory
                    sub_dir = Some(&Path::new(arg));
                } else {
                    // we already have a sub dir, this is just unexpected
                    error::err(&format!("unexpected argument: `{}`", arg));
                }
            },
        };
    }

    // Check a predicate on a value, and give an informative 
    // error message in the case of failure
    fn validate<F>(flag: &str, value: i32, valid: F, reminder: &str) 
        where F: Fn(i32) -> bool {
        if !valid(value) {
            error::err(&format!(
                "invalid value for {}: `{}` (remember: {})", flag, value, reminder));
        }
    }

    // validate both k & t: must be positive and 0 < k <= t
    let kt_remind = "0 < k <= t";
    validate("k", options.k, |k| k > 0 && k <= options.t, kt_remind);
    validate("t", options.t, |t| t > 0 && t >= options.k, kt_remind);

    // validate max pairs out
    match options.max_pairs_out {
        Some(max) => validate("--max-out", max, |m| m > 0, 
            "must be >0 submission pairs in output"),
        None => (),
    };

    if let Some(dir) = sub_dir {
        // return the submissions directory & updated options
        return (dir, options);
    } else {
        error::err("no submission directory given");
        panic!();   // won't get here
    }
}

// Print a help message explaining the command line interface & exit
fn print_help() {
    println!("{}", HELP_MSG);
    std::process::exit(0);
}

static HELP_MSG: &str = "\
Copy-detection for Pyret

USAGE:
    ./pyret-moss <SUBMISSIONS-DIR> [OPTIONS]

SUBMISSIONS-DIR indicates a directory containing submissions (either individual .arr
files or subdirectories of .arr files)

OPTIONS:
    --help                  Prints this help information
    --single-file-subs      Indicates that submissions will be treated as single .arr files
    -k <VALUE>              Sets the noise threshold, k
    -t <VALUE>              Sets the guarantee threshold, t
    --ignore <DIR>          Indicates submission matches with the .arr files in DIR should be ignored
    --max-out <VALUE>       Limits the number of submission pairs in the output analysis to VALUE
    -o <FILE>               Writes the analysis to FILE instead of stdout
    --verbose               More logging";