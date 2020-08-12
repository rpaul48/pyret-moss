#[macro_use] extern crate lazy_static;
extern crate regex;
#[macro_use] extern crate prettytable;
use std::path::PathBuf;
use crate::fingerprint::Fingerprint;
#[macro_use] mod error;
mod fingerprint;
mod normalize;
mod file_io;
mod phase_i;
mod phase_ii;
mod cli;
mod results;
mod io_redirect;

// Sub represents a student submission.
// Depending on whether input submissions are directories or
// indiv. files, the dir_name field will be Some or None
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Sub {
    pub dir_name: Option<PathBuf>,
    pub documents: Vec<Doc>
}

// Doc represents a file within a submission.
// Docs are initialized as Unprocessed (contents have not yet been
// read), and become Processed once they have been fingerprinted/line counted
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Doc {
    Unprocessed(PathBuf),
    Processed(PathBuf, Vec<Fingerprint>)
}

fn main() {

    // use io_redirect::*;
    // use gag::Redirect;
    // use std::path::Path;

    // let out = Path::new("test-dirs/tinker/output.txt");
    // let redirect = true;
    // let mut print_redirect: Option<Redirect<std::fs::File>> = None;
    
    // if redirect {
    //     print_redirect = Some(initialize_redirect(out));
    // }
    
    // println!("FILE: This should appear in the redirect file!");
    
    // if let Some(_) = print_redirect {
    //     end_redirect(&mut print_redirect);
    //     println!("CONSOLE: Redirect temporarily suspended!");
    
    //     confirm_continue("CONSOLE: waiting for confirmation");
    
    //     println!("CONSOLE: Starting redirect again...");
    //     resume_redirect(&mut print_redirect, &out);
    //     println!("FILE: again this should be in the redirect file");   
    // }


    // terminal formatting / tables tests:

    // use ansi_term::Colour::RGB;
    // use ansi_term::Colour::{Blue, Yellow};
    // use ansi_term::Style;

    // println!("sub1/ and sub2/: {}",
    //         if !redirect {
    //             RGB(77, 255, 77).bold().paint("3 matches")
    //         } else {
    //             Style::default().paint("3 matches")
    //         });
}