/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::NormText;
use crate::normalize::normalize;

// the base value used by the hash function, usually the size of the character set
static BASE: i64 = 256;

// the prime modulus for all hash calculations to be done under, which also represents the
// range of possible hash values in the output of finerprint() (0, PRIME_MODULUS]
static PRIME_MODULUS: i64 = 2147483647;

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

// the robust winnowing algorithm; takes in a Vec<i64> of hashes and returns the fingerprints,
// or a Vec<(i64, usize)>, which represents a subset of the input hashes paired with their index
// In each window select the minimum hash value. If possible break ties by selecting
// the same hash as the window one position to the left. If not, select the rightmost minimal hash.
// Save all selected hashes as the fingerprints of the document.
fn robust_winnow(hashed_kgrams: Vec<i64>, window_size: usize) -> Vec<(i64, usize)> {

    let max_window_index: usize = hashed_kgrams.len() as usize;
    let mut window_start: usize = 0;
    let mut window_end: usize = window_size;

    // the output Vec, to be populated
    let mut fingerprint_tuples: Vec<(i64, usize)> = Vec::new();
    let mut prev_fingerprint: Option<(i64, usize)> = None;

    // if the window size is greater than the number of hashed kgrams, return rightmost min kgram
    if window_end > max_window_index {
        // find the minimum hash(es) in hashed_kgrams
        let mut cur_mins: Vec<(i64, usize)> = Vec::new();

        for (i, hash) in hashed_kgrams.iter().enumerate() {
            let potential_fingerprint: (i64, usize) = (*hash, i);
            if cur_mins.is_empty() {
                cur_mins.push(potential_fingerprint);
            } else if hash < &cur_mins[0].0 {
                cur_mins = vec![potential_fingerprint]
            } else if hash == &cur_mins[0].0 {
                cur_mins.push(potential_fingerprint);
            }
        }

        //add the rightmost minimum hash tuple to the output Vec
        fingerprint_tuples.push(*cur_mins.last().unwrap());

    } else {
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

            // compare cur_mins and prev_fingerprint to see whether to update fingerprint_tuples
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
    }
    fingerprint_tuples
}

// a rolling hash function for a vector of strings
// assumes each "kgram" &str in the input Vec is of length k
fn rolling_hash(mut kgrams: Vec<&str>) -> Vec<i64> {
    let len = kgrams.len();

    // the output vector of hashes, which is returned when the entirety of the input is hashed
    let mut output: Vec<i64> = Vec::new();
    let mut prev_first_char: Option<char> = None;

    while output.len() < len {
        let cur_str: &str = kgrams[0];
        kgrams = kgrams[1..].to_vec();

        let mut cur_first_char: char =
            cur_str.chars().next().unwrap().to_lowercase().next().unwrap();
        let mut cur_last_char: char =
            cur_str.chars().last().unwrap().to_lowercase().next().unwrap();

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

                // calculates the current hash by removing the prev first character's component,
                // multiplying the remaining k-1 chars by BASE, and adding the new character;
                // PRIME_MODULUS is added to prev_hash to prevent underflow
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
fn hash(str: &str) -> i64 {
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
fn mod_exp(mut base: i64, mut exponent: i64, modulus: i64) -> i64 {
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
mod tests {
    use super::*;

    #[test]
    // tests that mod_exp performs modular exponentiation as expected
    fn mod_exp_test() {
        assert_eq!(mod_exp(2, 5, 1), 0, "zero when mod 1");
        assert_eq!(mod_exp(3, 3, 82), 27, "normal exponent when much less than mod");
        assert_eq!(mod_exp(3, 4, 82), 81, "normal exponent when less than mod");
        assert_eq!(mod_exp(2, 5, 32), 0, "zero when base^exp equals mod");
        assert_eq!(mod_exp(2, 6, 32), 0, "zero when base^exp divisible by mod");
        assert_eq!(mod_exp(3, 5, 82), 79, "wraps around when base^exp exceeds mod");
        assert_eq!(mod_exp(19, 7, 123), 112, "can wrap around mod several times");
    }

    #[test]
    // tests hash() and rolling_hash() for empty inputs
    fn empty_input_hash() {
        let mut empty: Vec<&str> = Vec::new();

        assert_eq!(hash(""), 0);
        assert_eq!(rolling_hash(empty), vec![]);
    }

    #[test]
    // tests that single character strings are always represented as their code points after
    // hashing, with the exception of capital letters having the same value as lowercase counterparts
    fn code_points() {
        assert_eq!(hash("!"), 33);
        assert_eq!(hash("M"), 109);
        assert_eq!(hash("m"), 109);
        assert_eq!(hash("é"), 233);
        assert_eq!(rolling_hash(vec!["!", "M", "m", "é"]), vec![33, 109, 109, 233]);
    }

    #[test]
    // tests that hash() and rolling_hash() are case insensitive
    fn case_insensitive() {
        let mut lowercase_kgrams: Vec<&str> = vec!["this is a te", "his is a tes", "is is a test"];
        let mut mixcase_kgrams: Vec<&str> = vec!["tHis IS a TE", "his is a tes", "IS IS A TEST"];
        let mut uppercase_kgrams: Vec<&str> = vec!["THIS IS A TE", "HIS IS A TES", "IS IS A TEST"];

        assert_eq!(hash("a"), hash("A"));
        assert_eq!(hash("hello"), hash("HeLLo"));
        assert_eq!(rolling_hash(lowercase_kgrams), rolling_hash(mixcase_kgrams.to_owned()));
        assert_eq!(rolling_hash(mixcase_kgrams.to_owned()), rolling_hash(uppercase_kgrams));
    }

    #[test]
    // tests that hash() and rolling_hash() consider the order of characters
    fn rearranged_characters() {
        let mut arrange_1: Vec<i64> = rolling_hash(vec!["aba", "bab", "abb"]);
        let mut arrange_2: Vec<i64> = rolling_hash(vec!["aab", "abb", "bba"]);

        assert_ne!(hash("abc"), hash("bac"));
        assert_ne!(hash("this is a test"), hash("a test this is"));
        assert_ne!(arrange_1[0], arrange_2[0]);
        assert_ne!(arrange_1[1], arrange_2[1]);
        assert_ne!(arrange_1[2], arrange_2[2]);
    }

    #[test]
    // tests that distinct calls to identical inputs to hash() and rolling_hash() yield identical outputs
    fn equal_input_equal_output() {
        let mut three_kgrams: Vec<&str> = vec!["abcd", "bcde", "cdef"];
        let mut three_kgrams_hashed: Vec<i64> = rolling_hash(three_kgrams);
        let mut three_kgrams_copy: Vec<&str> = vec!["abcd", "bcde", "cdef"];

        assert_eq!(hash("abcdefg"), hash("abcdefg"));
        assert_eq!(hash("@ 3 df KM34,;."), hash("@ 3 df KM34,;."));
        assert_eq!(rolling_hash(three_kgrams_copy), three_kgrams_hashed);
    }

    #[test]
    // tests that hash() and rolling_hash() do not overflow on lengthy inputs
    fn no_overflow() {
        let long_input: i64 = hash("The quick brown fox jumps over the lazy dog");
        let high_code_points: i64 = hash("ó { |~ û ÿ ©÷ ó { |~ û ÿ ©÷ ó { |~ û ÿ ©÷");
        let mut large_kgrams: Vec<i64> = rolling_hash(vec!["each string is pretty long in this V",
        "ach string is pretty long in this Ve", "ch string is pretty long in this Vec"]);

        assert_eq!(long_input < PRIME_MODULUS, true);
        assert_eq!(high_code_points < PRIME_MODULUS, true);
        assert_eq!(large_kgrams[0] < PRIME_MODULUS, true);
        assert_eq!(large_kgrams[1] < PRIME_MODULUS, true);
        assert_eq!(large_kgrams[2] < PRIME_MODULUS, true);
    }

    #[test]
    // tests that rolling_hash() does not allow underflow when the first character component is
    // greater than the rest
    fn no_underflow() {
        let underflow_test: Vec<i64> = rolling_hash(vec!["ÿ!0", "!0!"]);

        assert_eq!(underflow_test[1] > 0, true);
    }

    #[test]
    // tests the output Vec of the rolling_hash() is the same length as the input Vec
    fn rolling_hash_output_length() {
        assert_eq!(rolling_hash(vec!["ab", "bc", "cd", "de", "ef"]).len(), 5);
    }

    #[test]
    // tests that the rolling hash produces the same hash values as the naive hash
    fn hash_vs_rolling_hash() {
        let mut one_kgram: Vec<&str> = vec!["there is one kgram!"];
        let mut three_kgrams: Vec<&str> = vec!["abcde", "bcdef", "cdefg"];
        let mut special_chars: Vec<&str> = vec!["$ 1:", " 1:,", "1:,a", ":,aA"];
        let mut spec_indiv_hashes = vec![hash("$ 1:"), hash(" 1:,"), hash("1:,a"), hash(":,aA")];

        assert_eq!(rolling_hash(one_kgram), vec![hash("there is one kgram!")]);
        assert_eq!(rolling_hash(three_kgrams), vec![hash("abcde"), hash("bcdef"), hash("cdefg")]);
        assert_eq!(rolling_hash(special_chars), spec_indiv_hashes);
    }

    #[test]
    // tests that the min is properly selected from a single window
    fn single_window_winnow() {
        let input: Vec<i64> = vec![13, 4, 72, 3];
        let output: Vec<(i64, usize)> = vec![(3, 3)];

        assert_eq!(robust_winnow(input, 4), output, "single window to be winnowed");
    }

    #[test]
    // tests that each member is selected as a fingeprint when the window size is 1
    fn smallest_window_size() {
        let input: Vec<i64> = vec![2, 2, 13, 64, 64, 2];
        let output: Vec<(i64, usize)> = vec![(2, 0), (2, 1), (13, 2), (64, 3), (64, 4), (2, 5)];

        assert_eq!(robust_winnow(input, 1), output, "window size of 1");
    }

    #[test]
    // tests that the rightmost min hash is selected as expected (when the left may not be)
    fn rightmost_selected() {
        let input: Vec<i64> = vec![2, 2, 13, 64, 64, 2];
        let output: Vec<(i64, usize)> = vec![(2, 1), (13, 2), (64, 4), (2, 5)];

        assert_eq!(robust_winnow(input, 2), output, "window size of 2, breaks ties right min");
    }

    #[test]
    // tests that the leftmost min hash is selected when possible (when in prev window)
    fn leftmost_selected() {
        let input: Vec<i64> = vec![2, 2, 13, 64, 9, 2];
        let output: Vec<(i64, usize)> = vec![(2, 1), (9, 4), (2, 5)];

        assert_eq!(robust_winnow(input, 3), output, "window size of 3, breaks ties left min");
    }

    #[test]
    // tests a hand-verified lengthy example on various window sizes
    fn lengthy_hand_verified() {
        let input: Vec<i64> = vec![4743, 2048, 728, 2741, 2332, 95, 2941, 95, 3040, 1619,
            4384, 3591, 3567, 4851, 4634, 2588, 3936, 1957, 3980, 3988, 4718, 4225,
            2120, 2954, 4093, 2298, 1760, 2];

        // window size 2
        let output: Vec<(i64, usize)> = vec![(2048, 1), (728, 2), (2332, 4), (95, 5), (95, 7),
            (1619, 9), (3591, 11), (3567, 12), (4634, 14), (2588, 15), (1957, 17), (3980, 18),
            (3988, 19), (4225, 21), (2120, 22), (2954, 23), (2298, 25), (1760, 26), (2, 27)];

        assert_eq!(robust_winnow(input.to_owned(), 2), output);

        // window size 5
        let output: Vec<(i64, usize)> = vec![(728, 2), (95, 5), (95, 7), (1619, 9), (3567, 12),
            (2588, 15), (1957, 17), (2120, 22), (1760, 26), (2, 27)];

        assert_eq!(robust_winnow(input.to_owned(), 5), output);

        // window size 10
        let output: Vec<(i64, usize)> = vec![(95, 7), (1619, 9), (1957, 17), (1760, 26), (2, 27)];

        assert_eq!(robust_winnow(input.to_owned(), 10), output);

        // window size 20
        let output: Vec<(i64, usize)> = vec![(95, 7), (2, 27)];

        assert_eq!(robust_winnow(input.to_owned(), 20), output);
    }

    #[test]
    // tests robust_winnow() on the example provided in the "Winnowing: Local Algorithms for
    // Document Fingerprinting" paper
    fn paper_verified_robust_winnow() {
        let paper_example: Vec<i64> =
            vec![77, 74, 42, 17, 98, 50, 17, 98, 8, 88, 67, 39, 77, 74, 42, 17, 98];
        let expected_output: Vec<(i64, usize)> = vec![(17, 3), (17, 6), (8, 8), (39, 11), (17, 15)];

        assert_eq!(robust_winnow(paper_example, 4), expected_output);
    }
}
