/* fingerprint.rs: Document fingerprinting using robust winnowing */

use crate::normalize::LineMapping;

// A Fingerprint contains a hash of a k-gram within a document, 
// and the range of line numbers to which that k-gram corresponds
pub struct Fingerprint {
    hash: i32,
    lines: (i32, i32)
}

// computes the fingerprints of a normalized document, using robust winnowing
fn fingerprint(doc: String, lm: Box<LineMapping>) -> Vec<Fingerprint> {

}