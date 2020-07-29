/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::LineMapping;

// the noise threshold; matches shorter than it are not considered
static k: i32 = 5;

// the window size, w = t - k + 1, where t is the guarantee threshold, or the minimum substring length at which matches are guaranteed to be caught
static w: i32 = 4;

// A Fingerprint contains a hash of a k-gram within a document,
// and the range of line numbers to which that k-gram corresponds, inclusive
pub struct Fingerprint {
    hash: i32,
    lines: (i32, i32)
}

// computes the fingerprints of a normalized document, using robust winnowing
fn fingerprint(doc: String/*,  lm: Box<LineMapping> */) -> Vec<Fingerprint> {
    // construct k-grams, a Vec<str>
    // hash each k-gram, construct a vector of i32s
    // construct windows of hashes of length p
    // from windows, use winnowing to select fingerprints
    // pair fingerprints with line number



    let len = doc.len();
    // only attempt to fingerprint if the processed string is greater than the noise threshold
    if (len > k) {
        // construct k-grams, a Vec<str>
        let mut k-grams = Vec::new();
        let mut start: i32 = 0;
        let mut end: i32 = k;

        while ((end - 1) <= len) {
            k-grams.push(&doc[start..end]);
            start += 1; end += 1;
        }

        // rolling hash each k-gram, constructing a vector of i32s
    }
}
