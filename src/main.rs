// use std::io;
// use std::fs;
// use std::path::Path;

use crate::fingerprint::*;

use crate::normalize::*;

#[macro_use] extern crate lazy_static;
extern crate regex;

mod error;
mod fingerprint;
mod normalize;

use error::err;
use fingerprint::Fingerprint;

struct Submission {
    path: String,
    documents: Vec<Document>
}

struct Document {
    path: String,
    fingerprints: Vec<Fingerprint>
}

fn main() {

    /* print tests for fingerprint() and robust_winnow()
    // example input from Stanford MOSS paper
    //let input: Vec<i64> = vec![77, 74, 42, 17, 98, 50, 17, 98, 8, 88, 67, 39, 77, 74, 42, 17, 98];
    //println!("{:?}", robust_winnow(input));
    println!("tweesearch3 example test for fingerprint()!");
    let code: &str = "fun search(search-tweet :: Tweet, alot :: List<Tweet>,
    threshold :: Number) -> List<Tweet>:
  doc: ```searches a list of tweets for those with a at least a certain
       relevance with a tweet, ranking results by descending relevance```
  cases(List) alot:
    | empty => empty
    | link(_, _) =>
      relevant-tweets = get-relevant-tw-all(search-tweet, alot, threshold, alot)
      relevant-tweets.foldl(lam(tw, rest):
        insert-tweet(search-tweet, tw, rest, alot) end, empty)
  end
end";

    let nt: NormText = normalize(code);
    //println!("{:?}", nt);
    let fingerprints: Vec<Fingerprint> = fingerprint(nt);
    println!("{:?}", fingerprints); */


    /* print tests for hash functions
    println!("using naive hash");
    println!("a%Hd: {}", hash("a%Hd"));
    println!("%Hdd: {}", hash("%Hdd"));
    println!("Hdd%: {}", hash("Hdd%"));

    println!("using rolling hash");
    let mut abcde_3grams: Vec<&str> = Vec::new();
    abcde_3grams.push("a%Hd");
    abcde_3grams.push("%Hdd");
    abcde_3grams.push("Hdd%");

    println!("a%Hd, %Hdd, Hdd%: {:?}", rolling_hash(abcde_3grams));

    println!("------");
    println!("{}", hash(" "));
    println!("{}", hash(""));
    let mut vect: Vec<&str> = Vec::new();
    println!("{:?}", rolling_hash(vect)); */



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
