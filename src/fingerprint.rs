/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::LineMapping;

// the noise threshold; matches shorter than it are not considered
static k: i32 = 5;

// the window size, w = t - k + 1, where t is the guarantee threshold, or the minimum substring length at which matches are guaranteed to be caught
static w: i32 = 4;

// the (preferably prime) base value used by the hash function
static base: i32 = 5;

// A Fingerprint contains a hash of a k-gram within a document,
// and the range of line numbers to which that k-gram corresponds, inclusive
pub struct Fingerprint {
    hash: i32,
    lines: (i32, i32)
}

// computes the fingerprints of a normalized document, using robust winnowing
fn fingerprint(doc: String,  lm: Box<LineMapping>) -> Vec<Fingerprint> {
    // construct k-grams, a Vec<str>
    // hash each k-gram, construct a vector of i32s
    // construct windows of hashes of length p
    // from windows, use winnowing to select fingerprints
    // pair fingerprints with line number



    let len = doc.len() as i32;
    // only attempt to fingerprint if the processed string is greater than the noise threshold
    if len > k {
        // construct k-grams, a Vec<str>
        let mut k_grams = Vec::new();
        let mut start: i32 = 0;
        let mut end: i32 = k;

        while (end - 1) <= len {
            let next_k_gram = &doc[start as usize..end as usize];
            k_grams.push(next_k_gram);
            start += 1; end += 1;
        }

        // rolling hash each k-gram, constructing a vector of i32s
    }

    unimplemented!();
}

// a simple, non-rolling, recursive hash function for strings
pub fn hash(str: &str) -> i32 {
    if str.is_empty() {
        0
    } else {
        let len = str.len() as i32;

        let first = str.chars().next().unwrap();
        let rest = &str[1..];

        (first as i32 * (base ^ (len - 1))) + hash(rest)
    }
}

#[cfg(test)]
mod fingerprint_tests {
    use super::*;
    #[test]
    fn hash_test() {
        assert_eq!(hash(""), 0);
        assert_eq!(hash("a"), 485);
        assert_eq!(hash("b"), 490);
        assert_eq!(hash("abc"), 1566);
        assert_eq!(hash("bac"), 1569);
        assert_eq!(hash("abca"), 2149);
    }
}
