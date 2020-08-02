/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::NormText;

// the noise threshold; matches shorter than it are not considered
static k: i32 = 5;

// the guarantee threshold, or the min substring length at which matches are guaranteed to be caught
static t: i32 = 8;

// the window size
static w: i32 = t - k + 1;

// the (preferably prime) base value used by the hash function
static base: i64 = 31;

// whether to transform all characters to lowercase
static ALL_TO_LOWER: bool = true;

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
        let mut start: usize = 0;
        let mut end: usize = k as usize;

        while (end - 1) <= len as usize {
            let next_kgram = &doc[start..end];
            kgrams.push(next_kgram);
            start += 1; end += 1;
        }

        // rolling hash each k-gram, constructing a vector of i32s
        let mut hashed_kgrams = rolling_hash(kgrams);

        // checks windows of hashes of length w, uses robust winnowing to select fingerprints

        /*
        let mut window_start: usize = 0;
        let mut window_end: usize = w as usize;
        let max_window_index: i32 = hashed_kgrams.len() as i32; */




        // pair fingerprints with line number
        let mut fingerprints: Vec<Fingerprint> = Vec::new();
    }

    unimplemented!();
}

// the robust winnowing algorithm, which takes in a Vec<i64> of hashes and returns the fingerprints,
// or a Vec<(i64, usize)>, which represents a subset of the input hashes paired with their index
// In each window select the minimum hash value. If possible break ties by selecting
//the same hash as the window one position to the left. If not, select the rightmost minimal hash.
// Save all selected hashes as the fingerprints of the document.
pub fn robust_winnow(hashed_kgrams: Vec<i64>) -> Vec<(i64, usize)> {

    let max_window_index: usize = hashed_kgrams.len() as usize;
    let mut window_start: usize = 0;
    let mut window_end: usize = w as usize;

    let mut fingerprint_tuples: Vec<(i64, usize)> = Vec::new();
    let mut prev_fingerprint: Option<(i64, usize)> = None;

    // check all windows of size w in the hashed kgrams
    while (window_end - 1) <= max_window_index {
        let window = &hashed_kgrams[window_start..window_end];

        // find the minimum hash(es) of the current window
        let mut cur_mins: Vec<(i64, usize)> = Vec::new();
        let mut cur_window_index_counter: usize = 0;

        for hash in window.iter() {
            let index: usize = window_start + cur_window_index_counter;
            let potential_fingerprint: (i64, usize) = (*hash, index);

            if cur_mins.is_empty() {
                cur_mins.push(potential_fingerprint);
            } else if hash < &cur_mins[0].0 {
                cur_mins = vec![potential_fingerprint]
            } else if hash == &cur_mins[0].0 {
                cur_mins.push(potential_fingerprint);
            }
            cur_window_index_counter += 1;
        }

        // compare cur_mins with prev_fingerprint to determine whether to update fingerprint_tuples
        match prev_fingerprint {
            // if no fingerprints have been identified Pyret
            None => {
                let next_fingerprint_tuple: (i64, usize) = *cur_mins.last().unwrap();
                fingerprint_tuples.push(next_fingerprint_tuple);
                prev_fingerprint = Some(next_fingerprint_tuple);
            }
            Some(fp) => {
                // if none of cur_mins is the previous fingerprint, select the rightmost hash
                if !cur_mins.contains(&fp) {
                    let next_fingerprint_tuple: (i64, usize) = *cur_mins.last().unwrap();
                    fingerprint_tuples.push(next_fingerprint_tuple);
                    prev_fingerprint = Some(next_fingerprint_tuple);
                }
            }
        }
        window_start += 1; window_end += 1;
    }
    fingerprint_tuples
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

        let mut cur_first_char: char = cur_str.chars().next().unwrap();
        let mut cur_last_char: char = cur_str.chars().last().unwrap();

        // ensure both characters are lowercase if necessary
        if ALL_TO_LOWER {
            cur_first_char = cur_first_char.to_lowercase().next().unwrap();
            cur_last_char = cur_last_char.to_lowercase().next().unwrap();
        }

        match prev_first_char {
            // if the current iteration is the first string being hashed
            None => {
                // appends the hash of the first string to the output vector
                output.push(hash(cur_str));
            }
            // if at least one string has already been hashed
            Some(c) => {
                // calculates the hash of the current string using the previous hash
                let str_len = cur_str.chars().count() as u32;
                let prev_hash: &i64 = output.last().unwrap();
                let prev_first_char_component = (c as i64 * base.pow((str_len - 1) as u32));
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

        let mut first;
        if ALL_TO_LOWER {
            first = str.chars().next().unwrap().to_lowercase().next().unwrap() as i64;
        } else {
            first = str.chars().next().unwrap() as i64;
        }

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
        let mut three_kgrams: Vec<&str> = vec!["abcde", "bcdef", "cdefg"];
        let mut special_chars: Vec<&str> = vec!["$ 1:", " 1:,", "1:,a", ":,aA"];

        assert_eq!(rolling_hash(three_kgrams), vec![hash("abcde"), hash("bcdef"), hash("cdefg")]);
        assert_eq!(rolling_hash(special_chars), vec![hash("$ 1:"), hash(" 1:,"), hash("1:,a"), hash(":,aA")]);
    }
}
