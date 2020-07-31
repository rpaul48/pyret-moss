/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::NormText;

// the noise threshold; matches shorter than it are not considered
static k: i32 = 4;

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
pub fn fingerprint(nt: NormText) -> Vec<Fingerprint> {
    // construct k-grams, a Vec<str>
    // hash each k-gram, construct a vector of i32s
    // construct windows of hashes of length p
    // from windows, use winnowing to select fingerprints
    // pair fingerprints with line number

    let doc = nt.value;

    let len = doc.len() as i32;
    // only attempt to fingerprint if the processed string is greater than the noise threshold
    if len > k {
        // construct k-grams, a Vec<str>
        let mut k_grams: Vec<&str> = Vec::new();
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

// a rolling hash function for a vector of strings
pub fn roll_hash(mut kgrams: Vec<&str>) -> Vec<i32> {
    let len = kgrams.len();

    // the output vector of hashes, which is returned when the entirety of the input is hashed
    let mut output: Vec<i32> = Vec::new();
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
                let prev_hash: &i32 = output.last().unwrap();
                let prev_first_char_component = (c as i32 * base.pow((k - 1) as u32));
                let hash: i32 = ((prev_hash - prev_first_char_component) * base) + cur_last_char as i32;
                
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
pub fn hash(str: &str) -> i32 {
    if str.is_empty() {
        0
    } else {
        let len = str.len() as u32;

        let first = str.chars().next().unwrap() as i32;
        let rest = &str[1..];

        (first * base.pow(len - 1)) + hash(rest)
    }
}

#[cfg(test)]
mod fingerprint_tests {
    use super::*;
    #[test]
    fn hash_test() {
        assert_eq!(hash(""), 0);
    }
}
