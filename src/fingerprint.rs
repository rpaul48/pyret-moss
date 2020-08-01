/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::NormText;

// the noise threshold; matches shorter than it are not considered
static k: i32 = 4;

// the window size, w = t - k + 1, where t is the guarantee threshold,
// or the minimum substring length at which matches are guaranteed to be caught
static w: i32 = 4;

// the (preferably prime) base value used by the hash function
static base: i64 = 5;

// A Fingerprint contains a hash of a k-gram within a document,
// and the range of line numbers to which that k-gram corresponds, inclusive
pub struct Fingerprint {
    hash: i64,
    lines: (i32, i32)
}

// computes the fingerprints of a normalized document, using robust winnowing
pub fn fingerprint(nt: NormText) -> Vec<Fingerprint> {

    let doc = nt.value;
    let len = doc.chars().count() as i32;

    // only attempt to fingerprint if the processed string is greater than the noise threshold
    if len > k {
        // construct k-grams, a Vec<str>
        let mut kgrams: Vec<&str> = Vec::new();
        let mut start: i32 = 0;
        let mut end: i32 = k;

        while (end - 1) <= len {
            let next_kgram = &doc[start as usize..end as usize];
            kgrams.push(next_kgram);
            start += 1; end += 1;
        }

        // rolling hash each k-gram, constructing a vector of i32s
        let mut hashed_kgrams = rolling_hash(kgrams);

        // construct windows of hashes of length w
        // from windows, use winnowing to select fingerprints
        // pair fingerprints with line number
    }

    unimplemented!();
}

// a rolling hash function for a vector of strings
// assumes each "kgram" &str in the input Vec is of length k
pub fn rolling_hash(mut kgrams: Vec<&str>) -> Vec<i64> {
    let len = kgrams.len();

    // the output vector of hashes, which is returned when the entirety of the input is hashed
    let mut output: Vec<i64> = Vec::new();
    let mut prev_first_char: Option<char> = None;

    while output.len() < len {
        //let cur_str: &str = kgrams.pop().unwrap();
        let cur_str: &str = kgrams[0];
        kgrams = kgrams[1..].to_vec();
        let cur_first_char: char = cur_str.chars().next().unwrap();
        let cur_last_char: char = cur_str.chars().last().unwrap();

        match prev_first_char {
            // if the current iteration is the first string being hashed
            None => {
                // appends the hash of the first string to the output vector
                output.push(hash(cur_str));
            }
            // if at least one string has already been hashed
            Some(c) => {
                // calculates the hash of the current string using the previous hash
                let prev_hash: &i64 = output.last().unwrap();
                let prev_first_char_component = (c as i64 * base.pow((k - 1) as u32));
                let hash: i64 = ((prev_hash - prev_first_char_component) * base) + cur_last_char as i64;

                // appends the hash of the current string to the output vector
                output.push(hash);
            }
        }
        //sets prev_first_char equal to the first character of the current string
        prev_first_char = Some(cur_first_char);
    }
    output
}

// a simple, non-rolling, recursive hash function for strings
// only matches the output of rolling_hash() when the input str is of length k
pub fn hash(str: &str) -> i64 {
    if str.is_empty() {
        0
    } else {
        let len = str.chars().count() as u32;
        let first = str.chars().next().unwrap() as i64;
        let rest = &str[1..];

        // computes a hash value corresponding to the first character of the input string
        // adds to a recursive call to the hash values of the remaining components of the string
        (first * base.pow(len - 1)) + hash(rest)
    }
}

#[cfg(test)]
mod fingerprint_tests {
    use super::*;
    #[test]
    // tests the hash() function for basic attributes
    fn basic_hash_tests() {
        assert_eq!(hash(""), 0, "tests the empty string case");
        assert_eq!(hash("a"), 97, "tests that the hash of a single character string is a code point");
        assert_eq!(hash("abcdefg"), hash("abcdefg"), "equal outputs for equal inputs");

        assert_ne!(hash("a"), hash("A"), "hash is case sensitive");
        assert_ne!(hash("abc"), hash("bac"), "different hash for rearranged characters");
        assert_ne!(hash("ab"), hash("a b"), "hashes spaces");
    }

    #[test]
    // tests the rolling_hash() function for basic attributes
    fn basic_rolling_hash_tests() {
        let mut empty: Vec<&str> = Vec::new();
        let mut one_char: Vec<&str> = vec!["a"];
        let mut three_kgrams: Vec<&str> = vec!["abcd", "bcde", "cdef"];
        let mut three_kgrams_copy: Vec<&str> = vec!["abcd", "bcde", "cdef"];
        let three_kgrams_hashed = rolling_hash(three_kgrams);

        assert_eq!(rolling_hash(empty).len(), 0, "outputs an empty vector for an empty input vector");
        assert_eq!(rolling_hash(one_char)[0], 97, "hash of a single character is a code point");
        assert_eq!(three_kgrams_hashed.len(), 3, "outputs three hash values");
        assert_eq!(rolling_hash(three_kgrams_copy), three_kgrams_hashed, "duplicate vectors");
    }

    #[test]
    // tests that the rolling hash produces the same hash values as the naive hash
    fn hash_vs_rolling_hash() {
        let mut three_kgrams: Vec<&str> = vec!["abcd", "bcde", "cdef"];
        let mut special_chars: Vec<&str> = vec!["$ 1:", " 1:,", "1:,a", ":,aA"];

        assert_eq!(rolling_hash(three_kgrams), vec![hash("abcd"), hash("bcde"), hash("cdef")]);
        assert_eq!(rolling_hash(special_chars), vec![hash("$ 1:"), hash(" 1:,"), hash("1:,a"), hash(":,aA")]);
    }
}
