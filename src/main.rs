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
}