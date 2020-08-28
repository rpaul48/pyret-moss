/* cli.rs: Functions for providing the command-line interface */

use std::path::Path;
use std::collections::HashSet;

// OptArgs encodes important system parameters that have default values
// but can be set via the command line interface
#[derive(Debug, PartialEq)]
pub struct OptArgs<'a> {
    pub sub_mode: SubFileMode,                  // indicates whether subs are files or dirs
    pub k: i32,                                 // noise threshold
    pub t: i32,                                 // guarantee threshold
    pub match_threshold: f64,                   // include sub pairs whose match percentage is at least this big
    pub ignore_content_dir: Option<&'a Path>,   // dir of files indicating expected overlap to ignore
    pub ignore_files: Option<HashSet<String>>,  // filenames of files to ignore
    pub out_file: Option<&'a Path>,             // where the program's result summary will be written (default stdout)
    pub verbose: bool,                          // option to increase intensity of logging
    pub no_pauses: bool                         // if true, don't pause to confirm when rendering output pairs
}

// SubFileMode indicates how submissions should be found within 
// the directory argument the program is given:
//  1) Single assumes submissions are individual .arr files, and
//     will construct one Sub for each .arr file in the given dir
//  2) Multi assumes submissions are directories with multiple .arr files
//     within them, and will construct one Sub for each dir in the given dir.
#[derive(Debug, PartialEq)]
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
            match_threshold: 0.0f64,
            ignore_content_dir: None,
            ignore_files: None,
            out_file: None,
            verbose: false,
            no_pauses: false
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
        panic!("program received 0 arguments, somehow");
    } else if argc == 1 {
        err!("usage: {} [options] <submission-dir>. See --help for more.", &args[0]);
    }

    let mut options = OptArgs::default();   // start with default options
    let mut sub_dir: Option<&Path> = None;

    // unwrap the next argument or give a specific error if none available
    fn unwrap_next<'a>(flag: &str, next: Option<&'a String>) -> &'a String {
        if let Some(arg) = next {
            arg
        } else {
            err!("expected an argument for {}", flag);
        }
    }

    let mut iter = args.iter().skip(1); // skip first arg (path to program)

    while let Some(arg) = iter.next() {
        let arg = arg.as_str();
        match arg {
            "--help" | "-h" => print_help(&args[0]),
            "--single-file-mode" | "-s" => options.sub_mode = SubFileMode::Single,
            "--noise" | "-k" => {
                let k_str = unwrap_next(arg, iter.next());

                // only accept integer k > 0
                if let Ok(k) = k_str.parse::<i32>() {
                    if k > 0 {
                        options.k = k;
                        continue;
                    }
                }
                err!("invalid value for noise threshold (k): `{}`", k_str);
            },
            "--guarantee" | "-t" => {
                let t_str = unwrap_next(arg, iter.next());

                // only accept integer t > 0
                if let Ok(t) = t_str.parse::<i32>() {
                    if t > 0 {
                        options.t = t;
                        continue;
                    }
                }
                err!("invalid value for guarantee threshold (t): `{}`", t_str);
            },
            "--output" | "-o" => {
                let out_file = unwrap_next(arg, iter.next());
                options.out_file = Some(&Path::new(out_file));
            },
            "--ignore-content" => {
                let ignore_content_dir = unwrap_next(arg, iter.next());
                options.ignore_content_dir = Some(&Path::new(ignore_content_dir));
            },
            "--ignore-files" => {
                let arg_string = unwrap_next(arg, iter.next());
                let mut ignore_files = HashSet::new();

                // add each filename to the set
                for file_name in arg_string.split(" ") {
                    if file_name.is_empty() {
                        err!("invalid argument to --ignore-files: `{}`", arg_string);
                    }

                    ignore_files.insert(String::from(file_name));
                }

                if ignore_files.len() == 0 {
                    err!("--ignore-files expected at least 1 filename to ignore");
                }

                options.ignore_files = Some(ignore_files);
            },
            "--match-threshold" => {
                let thresh_str = unwrap_next(arg, iter.next());

                if let Ok(match_threshold) = thresh_str.parse::<f64>() {
                    let match_threshold = match_threshold / 100.0f64;
                    options.match_threshold = match_threshold;
                } else {
                    err!("invalid value for --match-threshold: `{}`", thresh_str);
                }
            },
            "--verbose" | "-v" => options.verbose = true,
            "--no-pauses" | "-p" => options.no_pauses = true,
            _ => {
                // check for unrecognized flags
                if arg.starts_with('-') {
                    err!("unrecognized flag `{}`", arg);
                } else if let None = sub_dir {
                    // assume this is the submissions directory
                    sub_dir = Some(&Path::new(arg));
                } else {
                    // we already have a sub dir, this is just unexpected
                    err!("unexpected argument: `{}`", arg);
                }
            },
        };
    }

    // Check a predicate on a value, and give an informative error message 
    // in the case of failure. Call display() on value to prep it for printing
    fn validate<F, T, D>(flag: &str, value: &T, valid: F, display: D, reminder: &str) 
        where 
            F: Fn(&T) -> bool,
            T: std::fmt::Display,
            D: Fn(&T) -> T {
        if !valid(value) {
            err!("invalid value for {}: `{}` (remember: {})", flag, display(value), reminder);
        }
    }

    // validate both k & t: must be positive and 0 < k <= t
    let kt_remind = "0 < k <= t";
    validate("noise threshold (k)", &options.k, |&k| k > 0 && k <= options.t, 
        |&k| k, kt_remind);
    validate("guarantee threshold (t)", &options.t, |&t| t > 0 && t >= options.k, 
        |&t| t, kt_remind);

    // validate match threshold
    validate("--match-threshold", &options.match_threshold, |&t| t >= 0.0 && t <= 1.0, 
        |&m| m * 100.0, "must be a percentage value (0-100)");

    if let Some(dir) = sub_dir {
        // return the submissions directory & updated options
        return (dir, options);
    } else {
        err!("no submission directory given");
    }
}

// Print a help message explaining the command line interface & exit.
// Format the message so that whatever executable you used to run the help
// command is what appears in the directions
fn print_help(exec: &String) {
    // this is no good but I want to be able to use it as a formatting 
    // string and I don't want to write out the whitespace explicitly
    println!(r#"
Copy-detection for Pyret

Usage:
    {} <SUBMISSIONS-DIR> [OPTIONS]

SUBMISSIONS-DIR indicates a directory containing submissions.

Submissions can be either 
    1) individual .arr files (single-file mode)
    2) subdirectories of .arr files (multi-file mode (default))

OPTIONS:
    -h, --help                              Prints this help information
    -s, --single-file-mode                  Submissions are assumed to be single .arr files
    -k, --noise <VALUE>                     Sets the noise threshold
    -t, --guarantee <VALUE>                 Sets the guarantee threshold
        --ignore-content <DIR>              Ignore portions of submissions that match any file's content in DIR
        --ignore-files "<FILE>[ <FILE>]"    Ignore submission files with the given names
        --match-threshold <VALUE>           Only report submission pairs with pair percentile at least VALUE (0-100)
    -o, --output <FILE>                     Write analysis to FILE instead of stdout
    -v, --verbose                           More logging
    -p, --no-pauses                         Don't pause for confirmation to continue when rendering results

Note: abbreviated flags cannot be combined

For more detailed info see https://github.com/rpaul48/pyret-moss
"#, 
    exec);

    std::process::exit(0);
}


#[cfg(test)]
mod tests {
    use super::*;

    // convert a Vec<&str> into Vec<String>, for convenience
    fn to_vec_string(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_args_no_optionals() {
        {
            let args = to_vec_string(vec![
                "./pyret-moss",
                "/home/user/Desktop/submissions"
            ]);

            let (sub_dir, opt_args) = parse_args(&args);

            assert_eq!(sub_dir, Path::new("/home/user/Desktop/submissions"));
            assert_eq!(opt_args, OptArgs::default());
        }
        {
            let args = to_vec_string(vec![
                "pmoss",
                "./here/are/the/submissions"
            ]);

            let (sub_dir, opt_args) = parse_args(&args);

            assert_eq!(sub_dir, Path::new("./here/are/the/submissions"));
            assert_eq!(opt_args, OptArgs::default());
        }
    }

    #[test]
    fn parse_args_with_options() {
        {
            let args = to_vec_string(vec![
                "./pyret-moss",
                "-k",
                "10",
                "--ignore-content",
                "./dirs/ignore",
                "./subs"
            ]);

            let (sub_dir, opt_args) = parse_args(&args);

            assert_eq!(sub_dir, Path::new("./subs"));
            assert_eq!(opt_args, OptArgs {
                sub_mode: SubFileMode::Multi,
                k: 10,
                t: 20,
                ignore_content_dir: Some(&Path::new("./dirs/ignore")),
                ignore_files: None,
                match_threshold: 0.0,
                out_file: None,
                verbose: false,
                no_pauses: false
            });
        }
        {
            let args = to_vec_string(vec![
                "./pyret-moss",
                "~/submissions",
                "--match-threshold",
                "16.8",
                "--verbose",
                "-o",
                "~/Desktop/analysis.txt",
                "-k",
                "20",
                "-t",
                "25",
                "--single-file-mode",
                "--ignore-content",
                "./boilerplate",
                "--no-pauses"
            ]);

            let (sub_dir, opt_args) = parse_args(&args);

            assert_eq!(sub_dir, Path::new("~/submissions"));
            assert_eq!(opt_args, OptArgs {
                sub_mode: SubFileMode::Single,
                k: 20,
                t: 25,
                ignore_content_dir: Some(&Path::new("./boilerplate")),
                ignore_files: None,
                match_threshold: 0.168,
                out_file: Some(&Path::new("~/Desktop/analysis.txt")),
                verbose: true,
                no_pauses: true
            });
        }
        {
            let args = to_vec_string(vec![
                "./pyret-moss",
                "./submissions",
                "--ignore-files",
                "common.arr test.arr",
                "--verbose",
                "--no-pauses"
            ]);

            let (sub_dir, opt_args) = parse_args(&args);

            let mut ignore_files = HashSet::new();
            ignore_files.insert(String::from("common.arr"));
            ignore_files.insert(String::from("test.arr"));

            assert_eq!(sub_dir, Path::new("./submissions"));
            assert_eq!(opt_args, OptArgs {
                sub_mode: SubFileMode::Multi,
                k: 15,
                t: 20,
                ignore_content_dir: None,
                ignore_files: Some(ignore_files),
                match_threshold: 0.0,
                out_file: None,
                verbose: true,
                no_pauses: true
            });
        }
    }
}