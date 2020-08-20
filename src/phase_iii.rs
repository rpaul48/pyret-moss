/* Phase III: Find common substrings of fingerprints in a submission pair */

use std::collections::HashSet;
use std::cmp::{min, max};
use crate::phase_ii::SubPair;
use crate::fingerprint::Fingerprint;
use crate::{Sub, Doc};

// An Entry indicates a particular section of a 
// document within a submission.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SubString {
    size: usize,
    hashes: Vec<i64>,
    a_entry: Entry,
    b_entry: Entry
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
    Set up rows/cols fingerprint vectors for sub A and B
    substr_cache & chosen_substrs are both HashSets

    table = substring_table()   // do the initial finding of substrings

    chosen_substrings = choose_substrings(...)

    // Match formation
    construct map from hash vector to SubStrings that share that hash vector
    use map to construct Vec<Match> by combining SubString entries that share same hash vector
    order Vec<Match> by size
    */
}

type SubStrTable = Vec<Vec<usize>>;
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
fn substr_table(rows: &FpVec, cols: &FpVec) -> SubStrTable {
    let mut table = Vec::new();

    for (r, row_el) in rows.iter().enumerate() {
        table.push(Vec::new()); // add a new row vector

        for (c, col_el) in cols.iter().enumerate() {
            // if both are fingerprints with the same hash
            if let (Some(row_fp), Some(col_fp)) = (row_el, col_el) {
                if (row_fp.hash == col_fp.hash) {
                    // write top-left diagonal + 1 to this cell
                    let diag = table[r - 1][c - 1];
                    table[r].push(diag + 1);
                    continue;
                }
            }

            // if either is None, or hashes are unequal, write 0
            table[r].push(0);
        }
    }

    table
}

// Choose longest common substrings from the substring table such that each row/col 
// fingerprint has at least 1 of their longest common substrings in the chosen set
fn choose_substrs(rows: &FpVec, cols: &FpVec, table: SubStrTable) -> HashSet<SubString> {
    unimplemented!();
}

// Trace diagonally down/right from table[r][c] to construct a SubString 
// representing the substring that lies on that diagonal
fn trace_diagonal(table: &SubStrTable, dims: (&FpVec, &FpVec), 
    coord: (usize, usize), docs: (usize, usize)) -> SubString {

    let (rows, cols) = dims;
    let (mut r, mut c) = coord;

    // 1 indicates start of substring--shouldn't be called anywhere else
    if table[r][c] != 1 {
        panic!("tried to trace diagonal on cell with value {}", table[r][c]);
    }

    let mut hashes = Vec::new();
    let mut lines: Option<((i32, i32), (i32, i32))> = None;

    // while there's more diagonal to be processed
    while r < rows.len() && c < cols.len() && table[r][c] != 0 {
        let a_elt = rows[r].unwrap();
        let b_elt = cols[c].unwrap();

        hashes.push(a_elt.hash);    // hashes match, so arbitrarily add A's

        // update line ranges to extend maximally
        match lines {
            Some((a_lines, b_lines)) => {
                lines = Some((
                    (min(a_lines.0, a_elt.lines.0), max(a_lines.1, a_elt.lines.1)),
                    (min(b_lines.0, b_elt.lines.0), max(b_lines.1, b_elt.lines.1))));
            },
            None => {
                lines = Some((a_elt.lines, b_elt.lines));
            }
        };

        // move diagonally down/rightward
        r += 1;
        c += 1;
    }

    if let Some(lines) = lines {
        // construct the SubString
        return SubString {
            size: hashes.len(),
            hashes: hashes,
            a_entry: Entry {
                doc_idx: docs.0,
                lines: lines.0
            },
            b_entry: Entry {
                doc_idx: docs.1,
                lines: lines.1
            }
        };
    } else {
        panic!("no lines were found in tracing diagonal");
    }
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

    #[test]
    fn test_substr_table() {
        {
            // single document each
            let rows = vec![
                None,
                Some(Fingerprint { hash: 180, lines: (2, 3) }),
                Some(Fingerprint { hash: 17, lines: (6, 10) }),
                Some(Fingerprint { hash: 224, lines: (11, 13) }),
                Some(Fingerprint { hash: 61, lines: (20, 22) }),
                Some(Fingerprint { hash: 17, lines: (24, 30) })
            ];

            let cols = vec![
                None,
                Some(Fingerprint { hash: 17, lines: (7, 14) }),
                Some(Fingerprint { hash: 224, lines: (26, 29) }),
                Some(Fingerprint { hash: 180, lines: (34, 39) }),
                Some(Fingerprint { hash: 17, lines: (46, 50) })
            ];

            let exp_table = vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0],
                vec![0, 1, 0, 0, 2],
                vec![0, 0, 2, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1]
            ];

            assert_eq!(substr_table(&rows, &cols), exp_table);
        }
        {
            // multiple documents
            let rows = vec![
                None,
                Some(Fingerprint { hash: 37, lines: (2, 3) }),
                Some(Fingerprint { hash: 22, lines: (6, 10) }),
                None,
                Some(Fingerprint { hash: 11, lines: (11, 13) }),
                Some(Fingerprint { hash: 6, lines: (20, 22) }),
                Some(Fingerprint { hash: 22, lines: (24, 30) })
            ];

            let cols = vec![
                None,
                Some(Fingerprint { hash: 5, lines: (3, 8) }),
                None,
                Some(Fingerprint { hash: 22, lines: (9, 12) }),
                Some(Fingerprint { hash: 11, lines: (14, 14) }),
                Some(Fingerprint { hash: 6, lines: (15, 16) }),
                None,
                Some(Fingerprint { hash: 6, lines: (17, 20) }),
                Some(Fingerprint { hash: 14, lines: (21, 28) }),
                Some(Fingerprint { hash: 11, lines: (28, 28) }),
            ];

            let exp_table = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 2, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
            ];

            assert_eq!(substr_table(&rows, &cols), exp_table);
        }
        {
            // longer substring
            let rows = vec![
                None,
                Some(Fingerprint { hash: 1, lines: (2, 3) }),
                Some(Fingerprint { hash: 2, lines: (6, 10) }),
                Some(Fingerprint { hash: 3, lines: (11, 13) }),
                Some(Fingerprint { hash: 4, lines: (20, 22) }),
                Some(Fingerprint { hash: 5, lines: (24, 30) }),
                Some(Fingerprint { hash: 1, lines: (27, 31) })
            ];

            let cols = vec![
                None,
                Some(Fingerprint { hash: 1, lines: (3, 8) }),
                None,
                Some(Fingerprint { hash: 2, lines: (4, 9) }),
                Some(Fingerprint { hash: 3, lines: (5, 10) }),
                Some(Fingerprint { hash: 4, lines: (6, 11) }),
                Some(Fingerprint { hash: 5, lines: (7, 12) })
            ];

            let exp_table = vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 0],
                vec![0, 0, 0, 0, 0, 3, 0],
                vec![0, 0, 0, 0, 0, 0, 4],
                vec![0, 1, 0, 0, 0, 0, 0],
            ];

            assert_eq!(substr_table(&rows, &cols), exp_table);
        }
    }

    #[test]
    fn test_trace_diagonal() {
        {
            let rows = vec![
                None, 
                Some(Fingerprint { hash: 1, lines: (1, 5) }), 
                Some(Fingerprint { hash: 2, lines: (5, 7) }), 
                Some(Fingerprint { hash: 1, lines: (10, 15) }), 
                Some(Fingerprint { hash: 2, lines: (20, 31) })];

            let cols = vec![
                None, 
                Some(Fingerprint { hash: 2, lines: (3, 9) }), 
                Some(Fingerprint { hash: 1, lines: (10, 22) }), 
                Some(Fingerprint { hash: 2, lines: (18, 24) }), 
                None, 
                Some(Fingerprint { hash: 1, lines: (14, 17) }), 
                Some(Fingerprint { hash: 2, lines: (16, 19) }), 
                Some(Fingerprint { hash: 1, lines: (20, 22) })];

            let table = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 1, 0, 1],
                vec![0, 1, 0, 2, 0, 0, 2, 0],
                vec![0, 0, 2, 0, 0, 1, 0, 3],
                vec![0, 1, 0, 3, 0, 0, 2, 0]
            ];

            // trace at r=1, c=2
            assert_eq!(
                trace_diagonal(&table, (&rows, &cols), (1, 2), (0, 0)),
                SubString {
                    size: 2,
                    hashes: vec![1, 2],
                    a_entry: Entry {
                        doc_idx: 0,
                        lines: (1, 7)
                    },
                    b_entry: Entry {
                        doc_idx: 0,
                        lines: (10, 24)
                    }
                });

            // trace at r=1, c=5
            assert_eq!(
                trace_diagonal(&table, (&rows, &cols), (1, 5), (0, 1)),
                SubString {
                    size: 3,
                    hashes: vec![1, 2, 1],
                    a_entry: Entry {
                        doc_idx: 0,
                        lines: (1, 15)
                    },
                    b_entry: Entry {
                        doc_idx: 1,
                        lines: (14, 22)
                    }
                });

            // trace at r=4, c=1
            assert_eq!(
                trace_diagonal(&table, (&rows, &cols), (4, 1), (0, 0)),
                SubString {
                    size: 1,
                    hashes: vec![2],
                    a_entry: Entry {
                        doc_idx: 0,
                        lines: (20, 31)
                    },
                    b_entry: Entry {
                        doc_idx: 0,
                        lines: (3, 9)
                    }
                });
        }
        {
            let rows = vec![
                None, 
                Some(Fingerprint { hash: 100, lines: (12, 14) }), 
                Some(Fingerprint { hash: 200, lines: (13, 18) }), 
                Some(Fingerprint { hash: 300, lines: (20, 25) }), 
                Some(Fingerprint { hash: 400, lines: (24, 29) }),
                Some(Fingerprint { hash: 500, lines: (30, 41) })];

            let cols = vec![
                None, 
                Some(Fingerprint { hash: 100, lines: (2, 5) }), 
                None, 
                Some(Fingerprint { hash: 200, lines: (1, 3) }), 
                Some(Fingerprint { hash: 300, lines: (4, 5) }), 
                Some(Fingerprint { hash: 400, lines: (7, 18) }),
                Some(Fingerprint { hash: 500, lines: (15, 22) })];

            let table = vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 0],
                vec![0, 0, 0, 0, 0, 3, 0],
                vec![0, 0, 0, 0, 0, 0, 4]
            ];

            // trace at r=1, c=1
            assert_eq!(
                trace_diagonal(&table, (&rows, &cols), (1, 1), (0, 0)),
                SubString {
                    size: 1,
                    hashes: vec![100],
                    a_entry: Entry {
                        doc_idx: 0,
                        lines: (12, 14)
                    },
                    b_entry: Entry {
                        doc_idx: 0,
                        lines: (2, 5)
                    }
                });

            // trace at r=2, c=3
            assert_eq!(
                trace_diagonal(&table, (&rows, &cols), (2, 3), (0, 1)),
                SubString {
                    size: 4,
                    hashes: vec![200, 300, 400, 500],
                    a_entry: Entry {
                        doc_idx: 0,
                        lines: (13, 41)
                    },
                    b_entry: Entry {
                        doc_idx: 1,
                        lines: (1, 22)
                    }
                });

            // trying to trace at a 0-cell is an error
            let result = std::panic::catch_unwind(|| trace_diagonal(&table, (&rows, &cols), (2, 1), (0, 0)));
            assert!(result.is_err());
        }
    }
}