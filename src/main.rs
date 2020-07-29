// use std::io;
// use std::fs;
// use std::path::Path;

mod fingerprint;
mod normalize;

use fingerprint::Fingerprint;

struct Document {
    path: String,
    fingerprints: Vec<Fingerprint>
}

struct Submission {
    path: String,
    documents: Vec<Document>
}

fn main() {
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
            call normalize() on doc text
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