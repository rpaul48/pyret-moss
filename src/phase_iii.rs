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

// In the DP table for our modified-longest-common-substring problem with hashes, 
// a Cell represents either the length of a common substring in a subproblem,
// or a reference to a cached SubString struct that runs through the current table cell.
#[derive(Debug, PartialEq, Clone)]
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
fn substr_table<'a>(rows: &FpVec, cols: &FpVec) -> SubStrTable<'a> {
    let mut table = Vec::new();

    for (r, row_el) in rows.iter().enumerate() {
        table.push(Vec::new()); // add a new row vector

        for (c, col_el) in cols.iter().enumerate() {
            // if both are fingerprints with the same hash
            if let (Some(row_fp), Some(col_fp)) = (row_el, col_el) {
                if (row_fp.hash == col_fp.hash) {
                    // get the count from the top-left cell
                    match table[r - 1][c - 1] {
                        Cell::Count(prev) => {
                            // write top-left diagonal + 1
                            table[r].push(Cell::Count(prev + 1));
                            continue;
                        },
                        _ => {
                            panic!("non-count cell encountered while constructing substring table");
                        }
                    };
                }
            }

            // if either is None, or hashes are unequal, write 0
            table[r].push(Cell::Count(0));
        }
    }

    table
}

// Chooses longest common substrings that include fingerprints in the primary.
// (secondary is other dimension of the table)
// Updates cache with newly created substrings, adds chosen substrings to chosen_substrs
// Updates table to include references to substrings in the cache
fn choose_substrs<'a>(row_primary: bool, rows: &FpVec, cols: &FpVec, table: &'a mut SubStrTable<'a>,
    substr_cache: &'a mut HashSet<SubString>, chosen_substrs: &mut HashSet<&'a SubString>) {
    
    let (primary, secondary) = if row_primary { (rows, cols) } else { (cols, rows) };
    let mut prim_doc_idx = 0;
    let mut sec_doc_idx = 0;

    for (p, prim_elt) in primary.iter().enumerate() {
        // if document delimiter, increment doc idx & move on
        if prim_elt.is_none() && p > 0 {
            prim_doc_idx += 1;
            continue;
        }

        let mut max: Option<(usize, bool, &SubString)> = None;

        // find the max substring along the secondary dimension
        for (s, sec_elt) in secondary.iter().enumerate() {
            // if document delimiter, increment & continue
            if sec_elt.is_none() && s > 0 {
                sec_doc_idx += 1;
                continue;
            }

            let cell = if row_primary { &table[p][s] } else { &table[s][p] };

            let substr = match cell {
                Cell::Count(n) => {
                    if *n == 0 {
                        // not a substring--move on
                        continue;
                    } else {
                        let (r, c) = if row_primary { (p, s) } else { (s, p) };

                        let prim_entry = Entry {
                            doc_idx: prim_doc_idx,
                            lines: prim_elt.unwrap().lines
                        };
                        let sec_entry = Entry {
                            doc_idx: sec_doc_idx,
                            lines: sec_elt.unwrap().lines
                        };

                        let entries = if row_primary {
                            (prim_entry, sec_entry)
                        } else {
                            (sec_entry, prim_entry)
                        };

                        // substring not yet cached: compute & cache
                        trace_diagonal(table, r, c, rows, cols, entries, Vec::new(), substr_cache)
                    }
                },
                Cell::CachedSubStr(s) => s,
            };

            match max {
                Some((size, already_chosen, maxes)) => {
                    // if longer substring, accept
                    if substr.size > size {
                        max = Some((substr.size, chosen_substrs.contains(&substr), substr));

                    // if equivalent length, favor already chosen substrings
                    } else if substr.size == size && !already_chosen {
                        let chosen = chosen_substrs.contains(&substr);
                        if chosen {
                            max = Some((substr.size, chosen, substr));
                        }
                    }
                }
                None => {
                    // accept as max
                    max = Some((substr.size, chosen_substrs.contains(&substr), substr));
                }
            };
        }

        // if a max is found & not yet chosen
        if let Some((_, chosen, substr)) = max {
            if !chosen {
                chosen_substrs.insert(substr);
            }
        }
    }
}

// Trace diagonally down/right from table[row][col] to construct a SubString, storing
// it in the cache & adding a reference to it at every cell on the diagonal
fn trace_diagonal<'a>(table: &mut SubStrTable<'a>, r: usize, c: usize, 
    rows: &FpVec, cols: &FpVec, mut entries_so_far: (Entry, Entry), 
    mut hashes_so_far: Vec<i64>, substr_cache: &'a mut HashSet<SubString>) -> &'a SubString {
    // first, validate cell
    match table[r][c] {
        Cell::Count(n) => {
            if n == 0 { panic!("tried to trace diagonal on 0-count cell"); }

            // because count > 0, these both must be Some()
            let row_elt = rows[r].unwrap();
            let col_elt = cols[c].unwrap();

            // add current cell's shared hash to the hashes in this substring.
            // because count != 0, hash is shared, so arbitrarily use the row element's hash
            hashes_so_far.push(row_elt.hash);

            // compute new entries from previous (following the diagonal, entries 
            // take the minimum starting line & maximum ending line to delimit the 
            // line range over which the entry occurs)
            let (Entry { doc_idx: a_doc, lines: (a_min, a_max) },
                Entry { doc_idx: b_doc, lines: (b_min, b_max) }) = entries_so_far;

            entries_so_far = (
                Entry { doc_idx: a_doc, lines: (min(a_min, row_elt.lines.0), max(a_max, row_elt.lines.1)) },
                Entry { doc_idx: b_doc, lines: (min(b_min, col_elt.lines.0), max(b_max, col_elt.lines.1)) }
            );

            // get a reference to the substring the cell is a part of
            let ref_to_substr = {
                // if diagonally downward/rightward is out of bounds or 0
                if r + 1 >= rows.len() || c + 1 >= cols.len() || 
                    table[r + 1][c + 1] == Cell::Count(0) {
                    // diagonal has been followed all the way, construct a new substring
                    let substr = SubString {
                        size: hashes_so_far.len(),
                        hashes: hashes_so_far,
                        a_entry: entries_so_far.0,
                        b_entry: entries_so_far.1
                    };

                    // insert a copy into the cache
                    substr_cache.insert(substr.clone());

                    // get a reference the to just-inserted substring
                    // (the one in the cache, so no lifetime issues)
                    substr_cache.get(&substr).unwrap()

                // if still more substring to look at, recur on the diagonal
                } else {
                    trace_diagonal(table, r + 1, c + 1, rows, cols, entries_so_far, 
                        hashes_so_far, substr_cache)
                }
            };

            // cache reference in table & return it
            table[r][c] = Cell::CachedSubStr(ref_to_substr);
            return ref_to_substr;
        },
        _ => {
            panic!("tried to trace diagonal on a non-count cell");
        }
    };
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
        use Cell::Count;
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
                vec![Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(1), Count(0)],
                vec![Count(0), Count(1), Count(0), Count(0), Count(2)],
                vec![Count(0), Count(0), Count(2), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(1), Count(0), Count(0), Count(1)]
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
                vec![Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(1), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(1), Count(0), Count(0), Count(0), Count(0), Count(1)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(0), Count(2), Count(0), Count(1), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(1), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0)]
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
                vec![Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(1), Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(1), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(2), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(0), Count(3), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(0), Count(0), Count(4)],
                vec![Count(0), Count(1), Count(0), Count(0), Count(0), Count(0), Count(0)],
            ];

            assert_eq!(substr_table(&rows, &cols), exp_table);
        }
    }

    #[test]
    fn test_trace_diagonal() {
        use Cell::{Count, CachedSubStr};
        {
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

            let mut table = vec![
                vec![Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(1), Count(0)],
                vec![Count(0), Count(1), Count(0), Count(0), Count(2)],
                vec![Count(0), Count(0), Count(2), Count(0), Count(0)],
                vec![Count(0), Count(0), Count(0), Count(0), Count(0)],
                vec![Count(0), Count(1), Count(0), Count(0), Count(1)]
            ];
            let mut exp_table = table.clone();

            let mut cache: HashSet<SubString> = HashSet::new();

            // ------ First Test: trace the substring starting at row=1, col=3 ------
            let entries1 = (
                Entry {
                    doc_idx: 0,
                    lines: rows[1].unwrap().lines
                },
                Entry {
                    doc_idx: 0,
                    lines: cols[3].unwrap().lines
                }
            );

            let out1 = trace_diagonal(&mut table, 1, 3, &rows, &cols, entries1, vec![], &mut cache);

            // substring expected to be constructed
            let substr1 = SubString {
                size: 2,
                hashes: vec![180, 17],
                a_entry: Entry {
                    doc_idx: 0,
                    lines: (2, 10)
                },
                b_entry: Entry {
                    doc_idx: 0,
                    lines: (34, 50)
                }
            };

            // check that it returns reference to constructed substring
            assert_eq!(out1, &substr1);

            // check that table was updated to use refs to cache substring now
            exp_table[1][3] = CachedSubStr(&substr1);
            exp_table[2][4] = CachedSubStr(&substr1);
            assert_eq!(table, exp_table);

            // check that cache includes new substring
            let mut exp_cache = HashSet::new(); exp_cache.insert(substr1.clone());
            assert_eq!(cache, exp_cache);


            // // ------ Second Test: trace the substring starting at row=5, col=1 ------
            // let entries2 = (
            //     Entry {
            //         doc_idx: 0,
            //         lines: rows[5].unwrap().lines
            //     },
            //     Entry {
            //         doc_idx: 0,
            //         lines: cols[1].unwrap().lines
            //     }
            // );

            // let out2 = trace_diagonal(&mut table, 5, 1, &rows, &cols, entries2, vec![], &mut cache);

            // // substring expected to be constructed
            // let substr2 = SubString {
            //     size: 1,
            //     hashes: vec![17],
            //     a_entry: Entry {
            //         doc_idx: 0,
            //         lines: (24, 30)
            //     },
            //     b_entry: Entry {
            //         doc_idx: 0,
            //         lines: (7, 14)
            //     }
            // };

            // // check that it returns reference to constructed substring
            // assert_eq!(out2, &substr2);

            // // check that table was updated to use refs to cache substring now
            // exp_table[5][1] = CachedSubStr(&substr2);
            // assert_eq!(table, exp_table);

            // // check that cache includes new substring
            // exp_cache.insert(substr2.clone());
            // assert_eq!(cache, exp_cache);
        }
    }
}