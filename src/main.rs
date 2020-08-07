#[macro_use] extern crate lazy_static;
extern crate regex;

mod fingerprint;
mod normalize;
mod file_io;
mod phase_i;
mod error;

/*
    User-available parameters:
        - ignore boilerplate code (indicate a dir)
        - single-dir mode: submissions are assumed to be each 1 doc
        - k: noise threshold
        - t: guarantee threshold
        - result location: where the program's result summary will be written (default stdout)
        - limit max number of pairs of subs to report on in output
*/

use phase_i::make_ignore_set;
use std::path::Path;

fn main() {
    match make_ignore_set(&Path::new("./test-dirs/ignore")) {
        Ok(v) => println!("{:?}", v),
        Err(e) => panic!("Error: {:?}", e),
    };
}
