/* Phase III: Find common substrings of fingerprints in a submission pair */

use std::collections::HashSet;
use crate::phase_ii::SubPair;
use crate::fingerprint::Fingerprint;
use crate::{Sub, Doc};

// An Entry indicates a particular section of a 
// document within a submission.
#[derive(Debug)]
pub struct Entry {
    pub doc_idx: usize,
    pub lines: (i32, i32)
}

// A Match indicates a set of entries from submission A which all share 
// a particular string of fingerprint hashes with a set of entries from B.
#[derive(Debug)]
pub struct Match {
    pub size: usize,
    pub a_entries: HashSet<Entry>,
    pub b_entries: HashSet<Entry>
}

// A SubString represents a shared string of fingerprints
// between submissions A and B.
#[derive(Debug)]
struct SubString {
    size: usize,
    hashes: Vec<i32>,
    a_entry: Entry,
    b_entry: Entry
}

// In the DP table for our modified-longest-common-substring problem with hashes, 
// a Cell represents either the length of a common substring in a subproblem,
// or a reference to a cached SubString struct that runs through the current table cell.
enum Cell<'a> {
    Count(usize),
    CachedSubStr(&'a SubString)
}

// Analyzes a pair of submissions to determine how overlap should be reported.
// Matches (each of which is backed by a common substring of fingerprint hashes)
// are selected such that the following property holds:
// 
// If a fingerprint it shared, then a longest common substring of hashes 
// that *includes that fingerprint* appears as a match in the output. 
//
// It's possible that a shared fingerprint appear in more than one match (it 
// may be part of the LCS that includes some other fingerprint), but it must 
// appear at least once.
pub fn analyze_pair(pair: SubPair) -> Vec<Match> {
    unimplemented!();
    /*
    Set up fingerprint vectors for sub A and B
    substr_cache & chosen_substrs are both HashSets

    table = substring_table()   // do the initial finding of substrings

    choose_substrs(primary = rows (A), secondary = cols (B))    // choose substrings for Sub A fps

    choose_substrs(primary = cols (B), secondary = rows (A))    // choose substrings for Sub B fps

    construct map from hash vector to SubStrings that share that hash vector
    use map to construct Vec<Match> by combining SubString entries that share same hash vector
    order Vec<Match> by size
    */
}

type SubStrTable<'a> = Vec<Vec<Cell<'a>>>;
type FpVec = Vec<Option<Fingerprint>>;


// Produce a vector of Options of all fingerprints in the given submission, 
// with different documents delimited by None
fn flatten_docs(sub: &Sub) -> FpVec {
    let mut flat: Vec<Option<Fingerprint>> = Vec::new();

    for doc in sub.documents.iter() {
        flat.push(None);    // add None to delimit documents
        match doc {
            Doc::Processed(_, fps) => {
                for fp in fps.iter() { flat.push(Some(*fp)); }
            }
            Doc::Unprocessed(_) => {
                panic!("unprocessed document encountered while flattening (flatten_docs)");
            }
        };
    }

    flat
}

// Populates the DP table for longest common substring, using rows
// & cols as the strings (documents are delimited by None)
fn substr_table<'a>(rows: FpVec, cols: FpVec) -> SubStrTable<'a> {
    unimplemented!();
}

// Chooses longest common substrings that include fingerprints in the primary.
// (secondary is other dimension of the table)
// Updates cache with newly created substrings, adds chosen substrings to chosen_substrs
// Updates table to include references to substrings in the cache
fn choose_substrs(primary: &FpVec, secondary: &FpVec, table: &mut SubStrTable,
    substr_cache: &mut HashSet<SubString>, chosen_substrs: &mut HashSet<SubString>) {
    unimplemented!();
}

// Trace diagonally down/right from table[row][col] to construct a SubString, storing
// it in the cache & adding a reference to it at every cell on the diagonal
fn trace_diagonal<'a>(table: &mut SubStrTable, row: usize, col: usize, 
    prev_entries: (Entry, Entry), prev_hashes: Vec<i32>, 
    primary: &FpVec, substr_cache: &'a mut HashSet<SubString>) -> &'a SubString {
    
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_flatten_docs() {
        {
            let s = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from(""), vec![
                        Fingerprint { hash: 17, lines: (1, 2) },
                        Fingerprint { hash: 38, lines: (3, 7) },
                        Fingerprint { hash: 22, lines: (14, 18) }
                    ]),
                    Doc::Processed(PathBuf::from(""), vec![
                        Fingerprint { hash: 889, lines: (3, 3) },
                        Fingerprint { hash: 24, lines: (4, 7) },
                        Fingerprint { hash: 105, lines: (7, 10) }
                    ]),
                    Doc::Processed(PathBuf::from(""), vec![
                        Fingerprint { hash: 98, lines: (1, 5) }
                    ])
                ]
            };

            assert_eq!(
                flatten_docs(&s),
                vec![
                    None,
                    Some(Fingerprint { hash: 17, lines: (1, 2) }),
                    Some(Fingerprint { hash: 38, lines: (3, 7) }),
                    Some(Fingerprint { hash: 22, lines: (14, 18) }),
                    None,
                    Some(Fingerprint { hash: 889, lines: (3, 3) }),
                    Some(Fingerprint { hash: 24, lines: (4, 7) }),
                    Some(Fingerprint { hash: 105, lines: (7, 10) }),
                    None,
                    Some(Fingerprint { hash: 98, lines: (1, 5) })
                ]);
        }
        {
            let s = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from(""), vec![
                        Fingerprint { hash: 3812, lines: (31, 40) },
                        Fingerprint { hash: 4722, lines: (40, 43) },
                        Fingerprint { hash: 2139, lines: (42, 49) },
                        Fingerprint { hash: 1274, lines: (45, 62) },
                        Fingerprint { hash: 2347, lines: (55, 81) }
                    ])
                ]
            };

            assert_eq!(
                flatten_docs(&s),
                vec![
                    None,
                    Some(Fingerprint { hash: 3812, lines: (31, 40) }),
                    Some(Fingerprint { hash: 4722, lines: (40, 43) }),
                    Some(Fingerprint { hash: 2139, lines: (42, 49) }),
                    Some(Fingerprint { hash: 1274, lines: (45, 62) }),
                    Some(Fingerprint { hash: 2347, lines: (55, 81) })
                ]);
        }
        {
            let s = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from(""), vec![]),
                    Doc::Processed(PathBuf::from(""), vec![
                        Fingerprint { hash: 41, lines: (31, 40) },
                        Fingerprint { hash: 28, lines: (40, 43) }
                    ])
                ]
            };

            assert_eq!(
                flatten_docs(&s),
                vec![
                    None,
                    None,
                    Some(Fingerprint { hash: 41, lines: (31, 40) }),
                    Some(Fingerprint { hash: 28, lines: (40, 43) })
                ]);
        }
    }
}