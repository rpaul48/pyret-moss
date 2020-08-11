/* results.rs: Render findings of overlap between submissions, if any */

use crate::{Sub, Doc};
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::path::Path;
use crate::cli::SubFileMode;
use crate::phase_ii::SubPair;
use prettytable::Table;

// number of results to display before prompting the user to continue
const RESULT_BUFFER_SIZE: i32 = 50; 

// Given a vector of matched submission pairs ordered by amount of overlap, 
// render a message (to stdout or the given file) summarizing the overlaps
fn render_results(sub_pairs: Vec<SubPair>, out_file: Option<&Path>, mode: SubFileMode) {
    unimplemented!();
    /*
        if Some out_file: redirect println! temporarily

        if pairs_to_hashes is empty
            log "Aye, no overlap was found" & exit

        log "Avast ye, there be submission overlap!"

        for each (i, sub1, sub2) in pairs_to_hashes enumerate

            if i > 0 & i % RESULT_BUFFER_SIZE == 0
                pause for stdin to confirm
                if 'n', exit

            table = pair_table(sub1, sub2, fp_hashes)

            log "sub1/ and sub2/: 3 matches" depending on fp_hashes.len()
            table.printstd()
    */
}

// Generate a table summarizing fingerprint matches for a given pair of submissions
fn pair_table(pair: &SubPair, mode: &SubFileMode) -> Table {
    unimplemented!();
    // let mut t = Table::new();

    // let (a_name, b_name) = match mode {
    //     SubFileMode::Multi => 
    //     SubFileMode::Single => 
    // };

    /*
        let mut t = table::new

        add title row to table (if single file mode, use file name as submission name)

        let match_n = 1;

        for hash in fp_hashes (ordered):
            sub1_entry = format_line_numbers(sub1, hash, mode).join(\n)
            sub2_entry = format_line_numbers(sub2, hash, mode).join(\n)

            add row![match_n, sub1_entry, sub2_entry]
            match_n++
        
        return t
    */
}

// Generate a formatted string describing the lines (/files if multi-file
// submission) on which the indicated fingerprint occurs
fn format_line_numbers(sub: &Sub, hash: i64, mode: &SubFileMode) -> Vec<String> {
    let mut formatted = Vec::new();

    for doc in sub.documents.iter() {
        if let Doc::Processed(path, fps) = doc {
            let mut doc_line = String::new();

            // write doc filename to doc line in multi-file mode
            if let SubFileMode::Multi = mode {
                let fname = path.file_name().unwrap();
                doc_line.push_str(&format!("{} ", fname.to_str().unwrap()));
            }

            let lines = get_lines(doc, hash);   // get line ranges associated with this hash

            if lines.is_empty() { continue; }   // nothing to write

            // depending on number of lines found, write 'lines' or 'line'
            let (start, end) = lines[0];
            if lines.len() > 1 || (end - start > 0) {
                doc_line.push_str("lines ");
            } else {
                doc_line.push_str("line ");
            }

            let len = lines.len();

            // for each line range
            for (i, range) in lines.iter().enumerate() {
                let suffix = if i < len - 1 { ", " } else { "" };  // commas delimit ranges
                let (st, en) = range;

                // add either single line or line range
                if en - st == 0 {
                    doc_line.push_str(&format!("{}{}", st, suffix));
                } else {
                    doc_line.push_str(&format!("{}-{}{}", st, en, suffix));
                }
            }

            // add this document's formatted line info to the vec
            formatted.push(doc_line);

        } else {
            panic!("unprocessed document encountered in format_line_numbers");
        }
    }

    formatted
}

// Construct a list of line ranges of all fingerprints in this doc
// that have the given hash
fn get_lines(doc: &Doc, hash: i64) -> Vec<(i32, i32)> {
    // insert a line range into a vector of line ranges, ensuring that
    // overlapping/consecutive ranges are coalesced into one 
    // (ie (1,4) & (3,5) -> (1,5) and (2,5) & (6,10) -> (2,10))
    // (assumes inserting into an *already coalesced* vector)
    fn coalesce_insert(lines: &mut Vec<(i32, i32)>, new: (i32, i32)) {
        if lines.is_empty() {
            lines.push(new);
        } else {
            // check last el for need to coalesce
            if let Some(lst) = lines.pop() {
                let (lst_start, lst_end) = lst;
                let (new_start, new_end) = new;

                // if new range overlaps or begins immediately after the
                // end of the last range, coalesce
                if new_start <= lst_end + 1 {
                    lines.push((lst_start, new_end));
                } else {
                    lines.push(lst);
                    lines.push(new);
                }
            } else {
                panic!("failed to pop from lines vector (get_lines)");
            }
        }
    }

    if let Doc::Processed(path, fps) = doc {
        let mut lines = Vec::new();

        // add lines for fingerprints that match the given hash
        for fp in fps.iter() {
            if fp.hash == hash {
                coalesce_insert(&mut lines, fp.lines);
            }
        }

        return lines;
    } else {
        panic!("unprocessed Doc encountered in get_lines");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::Fingerprint;
    use std::path::PathBuf;

    #[test]
    fn test_pair_table() {
        {
            let a = Sub {
                dir_name: Some(PathBuf::from("sub1/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("sub1/doc1.arr"), vec![
                        Fingerprint { hash: 17, lines: (1, 3) },
                        Fingerprint { hash: 20, lines: (5, 5) },
                        Fingerprint { hash: 17, lines: (6, 10) },
                        Fingerprint { hash: 11, lines: (10, 11) },
                        Fingerprint { hash: 11, lines: (12, 15) }
                    ]),
                    Doc::Processed(PathBuf::from("sub1/doc2.arr"), vec![
                        Fingerprint { hash: 51, lines: (21, 24) },
                        Fingerprint { hash: 20, lines: (25, 30) },
                        Fingerprint { hash: 17, lines: (44, 57) }
                    ])
                ]
            };
            let b = Sub {
                dir_name: Some(PathBuf::from("sub2/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("sub2/doc1.arr"), vec![
                        Fingerprint { hash: 11, lines: (5, 5) },
                        Fingerprint { hash: 17, lines: (8, 12) },
                        Fingerprint { hash: 40, lines: (12, 12) },
                        Fingerprint { hash: 11, lines: (17, 30) },
                        Fingerprint { hash: 33, lines: (29, 34) }
                    ]),
                    Doc::Processed(PathBuf::from("sub2/doc2.arr"), vec![
                        Fingerprint { hash: 12, lines: (3, 4) },
                        Fingerprint { hash: 28, lines: (4, 4) },
                        Fingerprint { hash: 20, lines: (8, 10) }
                    ])
                ]
            };
            let matches: HashSet<i64> = [11, 17, 20].iter().cloned().collect();

            let sp = SubPair {
                a: &a,
                a_percent: 45.,
                b: &b,
                b_percent: 78.,
                matches: matches,
                percentile: 55.
            };

            let exp_table = table!(
                ["", Fcbic->"sub1/ (45%)", Fcbic->"sub2/ (78%)"],
                [bc->"1", "doc1.arr lines 10-15", "doc1.arr lines 5, 17-30"],   // fp 11
                [bc->"2", "doc1.arr lines 1-3, 6-10\ndoc2.arr lines 44-57", "doc1.arr lines 8-12"], // fp 17
                [bc->"3", "doc1.arr line 5\ndoc2.arr lines 25-30", "doc2.arr lines 8-10"] // fp 20
            );

            assert_eq!(pair_table(&sp, &SubFileMode::Multi), exp_table);
        }
        {
            let a = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from("submissions/sub1.arr"), vec![
                        Fingerprint { hash: 28, lines: (4, 5) },
                        Fingerprint { hash: 12, lines: (5, 5) },
                        Fingerprint { hash: 28, lines: (11, 15) },
                        Fingerprint { hash: 28, lines: (16, 19) },
                        Fingerprint { hash: 28, lines: (18, 22) },
                        Fingerprint { hash: 17, lines: (30, 31) }
                    ])
                ]
            };
            let b = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from("submissions/sub2.arr"), vec![
                        Fingerprint { hash: 31, lines: (9, 15) },
                        Fingerprint { hash: 28, lines: (17, 17) },
                        Fingerprint { hash: 28, lines: (17, 29) },
                        Fingerprint { hash: 17, lines: (30, 31) },
                        Fingerprint { hash: 12, lines: (38, 42) }
                    ])
                ]
            };
            let matches: HashSet<i64> = [12, 17, 28].iter().cloned().collect();

            let sp = SubPair {
                a: &a,
                a_percent: 22.,
                b: &b,
                b_percent: 31.,
                matches: matches,
                percentile: 55.
            };

            let exp_table = table!(
                ["", Fcbic->"sub1.arr (22%)", Fcbic->"sub2.arr (31%)"],
                [bc->"1", "line 5", "lines 38-42"],   // fp 12
                [bc->"2", "lines 30-31", "lines 30-31"], // fp 17
                [bc->"3", "lines 4-5, 11-22", "lines 17-29"] // fp 28
            );

            assert_eq!(pair_table(&sp, &SubFileMode::Multi), exp_table);
        }
    }

    #[test]
    fn test_format_line_numbers() {
        {
            let sub = Sub {
                dir_name: Some(PathBuf::from("sub1/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("sub1/d1.arr"), vec![
                        Fingerprint { hash: 1, lines: (2, 2) },
                        Fingerprint { hash: 2, lines: (10, 10) },
                        Fingerprint { hash: 3, lines: (14, 20) },
                        Fingerprint { hash: 1, lines: (27, 29) }
                    ]),
                    Doc::Processed(PathBuf::from("sub1/d2.arr"), vec![
                        Fingerprint { hash: 2, lines: (3, 3) },
                        Fingerprint { hash: 2, lines: (7, 7) },
                        Fingerprint { hash: 7, lines: (8, 104) },
                        Fingerprint { hash: 1, lines: (155, 171) }
                    ])
                ]
            };

            let mode = SubFileMode::Multi;

            assert_eq!(format_line_numbers(&sub, 1, &mode), vec![
                "d1.arr lines 2, 27-29".to_string(),
                "d2.arr lines 155-171".to_string()
            ]);
            assert_eq!(format_line_numbers(&sub, 2, &mode), vec![
                "d1.arr line 10".to_string(),
                "d2.arr lines 3, 7".to_string()
            ]);
            assert_eq!(format_line_numbers(&sub, 3, &mode), vec![
                "d1.arr lines 14-20".to_string(),
            ]);
            assert_eq!(format_line_numbers(&sub, 7, &mode), vec![
                "d2.arr lines 8-104".to_string()
            ]);
        }
        {
            let sub = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from("submission6.arr"), vec![
                        Fingerprint { hash: 17, lines: (1, 7) },
                        Fingerprint { hash: 39, lines: (11, 11) },
                        Fingerprint { hash: 88, lines: (14, 14) },
                        Fingerprint { hash: 17, lines: (30, 35) },
                        Fingerprint { hash: 39, lines: (28, 34) },
                        Fingerprint { hash: 39, lines: (31, 37) }
                    ])
                ]
            };

            let mode = SubFileMode::Single;

            // no filenames
            assert_eq!(format_line_numbers(&sub, 17, &mode), vec![
                "lines 1-7, 30-35".to_string()
            ]);
            assert_eq!(format_line_numbers(&sub, 39, &mode), vec![
                "lines 11, 28-37".to_string()   // overlapping lines are coalesced
            ]);
            assert_eq!(format_line_numbers(&sub, 88, &mode), vec![
                "line 14".to_string()
            ]);
        }
    }

    #[test]
    fn test_get_lines() {
        {
            let doc = Doc::Processed(PathBuf::from("sub/docname.arr"), vec![
                Fingerprint { hash: 1, lines: (1, 7) },
                Fingerprint { hash: 41, lines: (10, 10) },
                Fingerprint { hash: 3, lines: (15, 21) },
                Fingerprint { hash: 1, lines: (21, 25) },
                Fingerprint { hash: 1, lines: (23, 31) },
                Fingerprint { hash: 18, lines: (40, 44) }
            ]);

            assert_eq!(get_lines(&doc, 1), vec![(1, 7), (21, 31)]);
            assert_eq!(get_lines(&doc, 41), vec![(10, 10)]);
            assert_eq!(get_lines(&doc, 18), vec![(40, 44)]);
        }
        {
            let doc = Doc::Processed(PathBuf::from("submission/doc.arr"), vec![
                Fingerprint { hash: 67, lines: (3, 3) },
                Fingerprint { hash: 200, lines: (11, 17) },
                Fingerprint { hash: 11, lines: (21, 21) },
                Fingerprint { hash: 11, lines: (21, 26) },
                Fingerprint { hash: 11, lines: (27, 30) },
                Fingerprint { hash: 67, lines: (40, 44) }
            ]);

            assert_eq!(get_lines(&doc, 67), vec![(3, 3), (40, 44)]);
            assert_eq!(get_lines(&doc, 11), vec![(21, 30)]);
            assert_eq!(get_lines(&doc, 200), vec![(11, 17)]);
        }
    }
}