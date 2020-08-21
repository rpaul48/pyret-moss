/* io_redirect.rs: Functionality for switching output between stdout & a file */

use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::Write;
use gag::Redirect;

// Begin redirecting stdout to the indicated file & return 
// the gag::Redirect if successful
pub fn initialize_redirect(file: &Path) -> Redirect<File> {
    // open/create the indicated file for writing
    let log = match 
        OpenOptions::new()
            .truncate(true) // only truncate on the first open
            .create(true)
            .write(true)
            .open(file) {
        Ok(f) => f,
        Err(e) => { err!("failed to open file {} for writing: {}", file.display(), e); },
    };

    Redirect::stdout(log).unwrap()
}

// Resume redirecting stdout to the indicated file (do NOT truncate)
// & update the given redirect to reflect it
pub fn resume_redirect(redirect: &mut Option<Redirect<File>>, file: &Path) {
    let log = match 
        OpenOptions::new()
            .append(true)   // append on further openings
            .write(true)
            .open(file) {
        Ok(f) => f,
        Err(e) => { err!("failed to resume writing to {}: {}", file.display(), e); }
    };

    match redirect {
        Some(_) => panic!("Attempted to redirect while already redirected"),
        None => *redirect = Some(Redirect::stdout(log).unwrap()),
    };
}

// End the given redirect & update its option wrapper to None
pub fn end_redirect(redirect: &mut Option<Redirect<File>>) {
    match redirect {
        Some(rd) => {
            drop(rd);
            *redirect = None;
        },
        None => panic!("Attempted to end redirect while not redirected."),
    };
}

// Print a message and enter an infinite loop until confirmation is 
// received from the user to proceed
pub fn confirm_continue() {
    loop {
        let mut confirm = String::new();

        print!("Continue? [y/n]: ");
        std::io::stdout().flush().unwrap();

        // take user input
        std::io::stdin()
            .read_line(&mut confirm)
            .expect("Failed to read line");

        let confirm = confirm.trim();

        if confirm == "y" {
            // continue
            break;
        } else if confirm == "n" {
            // exit early
            println!("Exiting!");
            std::process::exit(0);
        }
    }
}


#[cfg(test)]
mod tests {

    // NOTE: These tests involve redirecting print statements & will 
    // fail if run normally with `cargo test` 
    // 
    // To include these tests, run:
    // `cargo test --features "test_redirects" -- --nocapture`

    use super::*;
    use std::fs;

    // flush stdout so that print statements will go through
    #[allow(dead_code)]
    fn flush_stdout() {
        std::io::stdout().flush().unwrap();
    }

    // assert that a file's contents are the given string
    #[allow(dead_code)]
    fn expect_file_contents(file: &Path, conts: String) {
        // read file
        let contents = fs::read_to_string(file)
            .expect(&format!("Failed to read {}", file.display()));

        assert_eq!(contents, conts);
    }

    #[test]
    #[cfg(feature = "test_redirects")]
    fn test_initialize_redirect() {
        let file = Path::new("test-dirs/test/redirect/init_redirect.txt");

        {
            let text = "This text should be in init_redirect.txt";
            let _r = initialize_redirect(&file);

            println!("{}", &text);  // print, therefore writing to init_redirect.txt
            flush_stdout();

            expect_file_contents(&file, format!("{}\n", text));
        }
        {
            let text = "This content was redirected from stdout into this file";
            let _r = initialize_redirect(&file);

            print!("{}", &text);
            std::io::stdout().flush().unwrap();

            expect_file_contents(&file, String::from(text));
        }
    }

    #[test]
    #[cfg(feature = "test_redirects")]
    fn test_resume_redirect() {
        let file = Path::new("test-dirs/test/redirect/resume_redirect.txt");
        let initial_conts = "This file already has some text in it.\n";

        fs::write(file, &initial_conts)
            .expect("Failed to initially write to resume_redirect.txt");

        let text = "New text written after resuming.";
        let mut r: Option<Redirect<File>> = None;

        resume_redirect(&mut r, &file); // start redirecting to this file w/o erasing its contents

        println!("{}", &text);   // write text to file
        flush_stdout();

        // redirect should exist now
        assert!(match r {
            Some(_) => true,
            None => false,
        });

        // file should contain the printed text as well as its initial text
        expect_file_contents(&file, format!("{}{}\n", &initial_conts, &text));
    }

    #[test]
    #[cfg(feature = "test_redirects")]
    fn test_end_redirect() {
        let file = Path::new("test-dirs/test/redirect/end_redirect.txt");

        let during_redir = "This is printed during the redirect.";
        let after_redir = "This is printed AFTER the redirect was ended.";

        let mut r = Some(initialize_redirect(&file));

        println!("{}", &during_redir);
        flush_stdout();

        end_redirect(&mut r);

        println!("{}", &after_redir);
        flush_stdout();

        // redirect should be None now
        assert!(match r {
            Some(_) => false,
            None => true,
        });

        // file should only contain text from first println
        expect_file_contents(&file, format!("{}\n", &during_redir));
    }
}