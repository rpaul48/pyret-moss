// use std::io;
// use std::fs;
// use std::path::Path;

use crate::fingerprint::hash;
use crate::fingerprint::rolling_hash;

#[macro_use] extern crate lazy_static;
extern crate regex;

mod error;
mod fingerprint;
mod normalize;

use error::err;
use fingerprint::Fingerprint;

// represents a student submission
pub struct Sub {
    path: String,
    documents: Vec<Doc>
}

// represents a file within a Sub
pub struct Doc {
    path: String,
    fingerprints: Option<Vec<Fingerprint>>
}

/*
    User-available parameters:
        - ignore boilerplate code (indicate a dir)
        - single-dir mode: submissions are assumed to be each 1 doc
        - k: noise threshold
        - t: guarantee threshold
        - result location: where the program's result summary will be written (default stdout)
        - limit max number of pairs of subs to report on in output
*/

use fnv::FnvHashMap;

fn main() {

    let mut map = FnvHashMap::default();

    map.insert(157, "testing value");
    map.insert(21, "neat");

    println!("Out: {:?}", map.get(&157));
    println!("Out: {:?}", map.get(&21));

    

    // ask user for input directory of files
    //println!("Please enter the path to a directory of files:");

    //let mut folder_path_str = String::new();
    //io::stdin()
    //    .read_line(&mut folder_path_str)
    //    .expect("Failed to read input");

    //let str1 = &folder_path_str[..];
    //let folder_path = Path::new(str1);

    // let folder_path = Path::new("./test-dirs/txts");

    // for file in fs::read_dir(folder_path).unwrap() {
    //     println!("file path: {}", file.unwrap().path().display())
    // }
}

/*

main()
 
    hashbrown: (i32 -> &Submission)

    for each submission directory
        construct Submission struct for this sub

        for each document in this submission
            call normalize() on doc text -> normalized text & mapping
            call fingerprint() on normalized text & line mapping

            construct Document struct for this doc, add to Submission
            add all hashes used in this document to the growing set of hashes for this submission

        add ref to this submission to hashmap under each fingerprint within this submission (use the set)

    important fingerprints = pull all fingerprints with more than 1 associated submission

    hashbrown: ((&Submission, &Submission) -> Vec<i32>)

    for each important fingerprint
        for each possible pair of submissions associated with this print
            add this print to vec of prints mapped to by this submission pair

    order submission pairs by number of matches, take top n

    generate report for the user

*/
