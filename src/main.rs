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
    // use std::path::Path;
    // use std::fs::OpenOptions;
    // use gag::Redirect;
    // use std::fs::File;
    // use std::io::Write;

    // fn resume_redirect(print_redirect: &mut Option<Redirect<File>>, file: &Path) {
    //     let log = match 
    //         OpenOptions::new()
    //             .append(true)
    //             .write(true)
    //             .open(file) {
    //         Ok(f) => f,
    //         Err(e) => { err!("failed to resume writing to {}: {}", file.display(), e); }
    //     };

    //     match print_redirect {
    //         Some(_) => panic!("Attempted to redirect while already redirected"),
    //         None => *print_redirect = Some(Redirect::stdout(log).unwrap()),
    //     };
    // }

    // fn end_redirect(print_redirect: &mut Option<Redirect<File>>) {
    //     match print_redirect {
    //         Some(rd) => {
    //             drop(rd);
    //             *print_redirect = None;
    //         },
    //         None => panic!("Attempted to end redirect while not redirected."),
    //     };
    // }

    // let redirect = true;
    // let mut print_redirect: Option<Redirect<File>> = None;

    // let out = Path::new("test-dirs/tinker/output.txt");

    // if redirect {
    //     // initialize the redirect
    //     let log = match 
    //         OpenOptions::new()
    //             .truncate(true)
    //             .create(true)
    //             .write(true)
    //             .open(out) {
    //         Ok(f) => f,
    //         Err(e) => { err!("failed to open file {} for writing: {}", out.display(), e); },
    //     };


    //     print_redirect = Some(Redirect::stdout(log).unwrap());
    // }

    // println!("Redirected here!");

    // if let Some(_) = print_redirect {
    //     end_redirect(&mut print_redirect);

    //     println!("Redirect temporarily suspended!");

    //     loop {
    //         let mut input = String::new();

    //         print!("Continue? [y/n]: ");
    //         std::io::stdout().flush().unwrap();

    //         std::io::stdin()
    //             .read_line(&mut input)
    //             .expect("Failed to read line");

    //         let tr = input.trim();

    //         if tr == "y" {
    //             break;
    //         } else if tr == "n" {
    //             println!("Exiting!");
    //             std::process::exit(0);
    //         }
    //     }

    //     println!("Starting redirect again...");
    //     resume_redirect(&mut print_redirect, &out);
    //     println!("THIS SHOULD BE IN THE FILE");   
    // }













    //// terminal formatting / tables tests:

    // use ansi_term::Colour::RGB;
    // use ansi_term::Colour::{Blue, Yellow};
    // use ansi_term::Style;

    // println!("sub1/ and sub2/: {}",
    //         if !redirect {
    //             RGB(77, 255, 77).bold().paint("3 matches")
    //         } else {
    //             Style::default().paint("3 matches")
    //         });

    // let table = table!(
    //     ["", Fcbic->"sub1/ (67%)", Fcbic->"sub2/ (24%)"],
    //     [bc->"1", "doc1.arr lines 3-6, 18-21\ndoc2.arr lines 10-12", "doc1.arr lines 39-44"],
    //     [bc->"2", "doc1.arr lines 30-34", "doc1.arr lines 14-26"],
    //     [bc->"3", "doc4.arr line 6\ndoc2.arr lines 1-7", "doc3.arr lines 20-21\ndoc2.arr line 1\ndoc4.arr lines 34-39"]);

    // table.printstd();

    // println!();
    // println!("submission1.arr and submission2.arr: 3 matches");
    // let t2 = table!(
    //     ["", Fcbic->"submission1.arr (67%)", Fcbic->"submission2.arr (24%)"],
    //     [bc->"1", "lines 3-6, 18-21", "lines 39-44"],
    //     [bc->"2", "lines 30-34", "lines 14-26"],
    //     [bc->"3", "line 1-7", "lines 20-21, 34-39"]);
    // t2.printstd();
}