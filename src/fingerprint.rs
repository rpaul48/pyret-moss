/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::NormText;

// the base value used by the hash function, usually the size of the character set
static BASE: i64 = 256;

// the prime modulus for all hash calculations to be done under, which represents the range of
// possible hash values (0, PRIME_MODULUS]
static PRIME_MODULUS: i64 = 5381;

// A Fingerprint contains a hash of a k-gram within a document,
// and the range of line numbers to which that k-gram corresponds, inclusive
#[derive(Debug)]
pub struct Fingerprint {
    pub hash: i64,
    pub lines: (i32, i32)
}

// computes the Fingerprints of a normalized document using robust winnowing
// input k represents the noise threshold; matches shorter than it are not considered
// input t represents the the min substring length at which matches are guaranteed to be caught
pub fn fingerprint(nt: NormText, k: i32, t: i32) -> Vec<Fingerprint> {

    // the text field of the input NormText
    let doc: &String = &nt.value;
    let len: i32 = doc.chars().count() as i32;

    // the output Vec<Fingerprint>, to be populated if possible
    let mut fingerprints: Vec<Fingerprint> = Vec::new();

    // only attempt to fingerprint if the normalized string is greater than the noise threshold
    if len > k {
        // construct k-grams, a Vec<str>
        let mut kgrams: Vec<&str> = Vec::new();
        let mut start: usize = 0;
        let mut end: usize = k as usize;

        while end <= len as usize {
            let next_kgram = &doc[start..end];
            kgrams.push(next_kgram);
            start += 1; end += 1;
        }

        // rolling hash each k-gram, constructing a vector of i32s
        let mut hashed_kgrams = rolling_hash(kgrams);

        // the window size for winnowing
        let w: i32 = t - k + 1;

        // checks windows of hashes of length w, uses robust winnowing to select fingerprints
        let mut fingerprint_tuples: Vec<(i64, usize)> = robust_winnow(hashed_kgrams, w as usize);

        // combine fingerprint tuples with original line numbers, make Fingerprint structs
        for tuple in fingerprint_tuples.iter() {
            let hash: i64 = tuple.0;
            let start_line: i32 = nt.line_number(tuple.1 as i32);
            let end_line: i32 = nt.line_number(tuple.1 as i32 + k - 1);
            let fingerprint: Fingerprint = Fingerprint {
                hash: hash,
                lines: (start_line, end_line)
            };
            fingerprints.push(fingerprint);
        }
    }
    fingerprints
}

// the robust winnowing algorithm, which takes in a Vec<i64> of hashes and returns the fingerprints,
// or a Vec<(i64, usize)>, which represents a subset of the input hashes paired with their index
// In each window select the minimum hash value. If possible break ties by selecting
//the same hash as the window one position to the left. If not, select the rightmost minimal hash.
// Save all selected hashes as the fingerprints of the document.
pub fn robust_winnow(hashed_kgrams: Vec<i64>, window_size: usize) -> Vec<(i64, usize)> {

    let max_window_index: usize = hashed_kgrams.len() as usize;
    let mut window_start: usize = 0;
    let mut window_end: usize = window_size;

    let mut fingerprint_tuples: Vec<(i64, usize)> = Vec::new();
    let mut prev_fingerprint: Option<(i64, usize)> = None;

    // check all windows of size w in the hashed kgrams
    while window_end <= max_window_index {
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
        let cur_str: &str = kgrams[0];
        kgrams = kgrams[1..].to_vec();

        let mut cur_first_char: char = cur_str.chars().next().unwrap().to_lowercase().next().unwrap();
        let mut cur_last_char: char = cur_str.chars().last().unwrap().to_lowercase().next().unwrap();

        match prev_first_char {
            // if the current iteration is the first string being hashed
            None => {
                // appends the hash of the first string to the output vector
                output.push(hash(cur_str));
            }
            // if at least one string has already been hashed
            Some(c) => {
                // calculates the hash of the current string using the previous hash
                let str_len = cur_str.chars().count() as i64;
                let prev_hash: &i64 = output.last().unwrap();

                // calculates the portion represented by the previous hash's first character
                let prev_first_char_component =
                    (c as i64 * mod_exp(BASE, str_len - 1, PRIME_MODULUS)) % PRIME_MODULUS;

                // calculates the current hash by removing the previous first character's component,
                // multiplying the remaining k-1 chars by BASE, and adding the new character
                let hash: i64 = ((prev_hash + PRIME_MODULUS - prev_first_char_component) * BASE
                + cur_last_char as i64) % PRIME_MODULUS;

                // appends the hash of the current string to the output vector
                output.push(hash);
            }
        }
        //sets prev_first_char equal to the first character of the current string
        prev_first_char = Some(cur_first_char);
    }
    output
}

// a simple, non-rolling hash function for strings
// only matches the output of rolling_hash() when the input str is of length k
pub fn hash(str: &str) -> i64 {
    let len = str.chars().count() as usize;
    let mut hash_val: i64 = 0;

    // for each character c in the string, the value c multiplied by the modular exponent
    // of (length of string - index of character - 1) is added to hash_val
    for (i, c) in str.chars().enumerate() {
        let char_lowered: i64 = c.to_lowercase().next().unwrap() as i64;
        hash_val = (hash_val + (char_lowered * mod_exp(BASE, (len - i) as i64 - 1, PRIME_MODULUS))
        % PRIME_MODULUS) % PRIME_MODULUS;
    }

    hash_val
}

// a modular exponentiation function, which calculates the remainder when base is raised to
// the power of exponent and divided by modulus
pub fn mod_exp(mut base: i64, mut exponent: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        0
    } else {
        let mut output = 1;
        base = base % modulus;
        while exponent > 0 {
            if exponent % 2 == 1 {
                output = output * base % modulus;
            }
            exponent = exponent >> 1;
            base = base * base % modulus
        }
        output
    }
}

#[cfg(test)]
mod fingerprint_tests {
    use super::*;
    // tests for helper functions
    #[test]
    // tests the hash() function for basic attributes
    fn basic_hash_tests() {
        assert_eq!(hash(""), 0, "tests the empty string case");
        assert_eq!(hash("a"), 97, "tests that the hash of a single character string is its code point");
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
        let mut one_kgram: Vec<&str> = vec!["there is one kgram!"];
        let mut three_kgrams: Vec<&str> = vec!["abcde", "bcdef", "cdefg"];
        let mut special_chars: Vec<&str> = vec!["$ 1:", " 1:,", "1:,a", ":,aA"];

        assert_eq!(rolling_hash(one_kgram), vec![hash("there is one kgram!")]);
        assert_eq!(rolling_hash(three_kgrams), vec![hash("abcde"), hash("bcdef"), hash("cdefg")]);
        assert_eq!(rolling_hash(special_chars), vec![hash("$ 1:"), hash(" 1:,"), hash("1:,a"), hash(":,aA")]);
    }

    #[test]
    // tests robust_winnow() on hand-verified examples
    fn hand_verified_robust_winnow() {
        // single window to be winnowed
        let single_window: Vec<i64> = vec![13, 4, 72, 3];
        let expected_output: Vec<(i64, usize)> = vec![(3, 3)];

        assert_eq!(robust_winnow(single_window, 4), expected_output);



        // from Stanford MOSS paper
        let paper_example: Vec<i64> = vec![77, 74, 42, 17, 98, 50, 17, 98, 8, 88, 67, 39, 77, 74, 42, 17, 98];
        let expected_output: Vec<(i64, usize)> = vec![(17, 3), (17, 6), (8, 8), (39, 11), (17, 15)];

        assert_eq!(robust_winnow(paper_example, 4), expected_output);
    }

/*
    #[test]
    // tests fingerprint() on a hand-verified example (from Stanford MOSS paper)
    fn verified_fingerprint() {


    } */

}
