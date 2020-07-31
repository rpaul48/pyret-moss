/* normalize.rs: Pre-processer for Pyret programs to eliminate irrelevant features */

// A NormText stores the normalized text of some program and
// encodes line number information from the original
// file from which normalized version has been generated.
// (accessible from line_number method)
//
// line_ends[x] = y means that y is the index of the first char
// in the normalized text occurring after line x+1 in the original
#[derive(Debug, PartialEq)]
pub struct NormText {
    pub value: String,
    line_ends: Vec<i32>
}

impl NormText {
    // determine the line number in the original text that
    // a char at index i in the normalized text corresponds to
    pub fn line_number(i: i32) -> i32 {
        unimplemented!();
    }
}


// Remove/normalize any features from a program's text that
// shouldn't differentiate it from other programs:
//      1. normalize whitespace to a single space
//      2. normalize identifiers
//      3. remove type annotations
//      4. remove docstrings
//      5. remove comments
// Returns the normalized string & enough info to map parts
// of the normalized text to line numbers in the original
pub fn normalize(program: String) -> NormText {
    unimplemented!();
}
