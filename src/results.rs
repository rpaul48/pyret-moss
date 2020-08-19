/* results.rs: Render findings of overlap between submissions, if any */

use crate::{Sub, Doc};
use std::path::{Path, PathBuf, Component};
use crate::cli::SubFileMode;
use crate::phase_ii::SubPair;
use crate::io_redirect;
use prettytable::Table;

// number of results to display before prompting the user to continue
const RESULT_BUFFER_SIZE: usize = 1;

// Given a vector of matched submission pairs ordered by amount of overlap, 
// render a message (to stdout or the given file) summarizing the overlaps
pub fn render_results(sub_pairs: Vec<SubPair>, mode: &SubFileMode, 
    out_file: Option<&Path>, match_thresh: f64, total_pairs: usize, no_pauses: bool) {

    // if output filepath given, start redirecting stdout to that file
    let redirecting = out_file.is_some();
    let mut redirect = match out_file {
        Some(p) => Some(io_redirect::initialize_redirect(p)),
        None => None,
    };

    // if no submission pairs were found in Phase II, exit
    if sub_pairs.is_empty() {
        format::no_overlap_msg(redirecting);
        std::process::exit(0);
    }

    format::overlap_found_msg(redirecting);

    let total_pairs_rendering = sub_pairs.len();
    format::num_pairs_rendering(redirecting, match_thresh, total_pairs, total_pairs_rendering);

    // for each pair & its index
    for (i, pair) in sub_pairs.iter().enumerate() {
        // periodically, ask user for confirmation to continue rendering results
        if !no_pauses && i > 0 && i % RESULT_BUFFER_SIZE == 0 {
            // pause redirection of stdout
            if redirecting { io_redirect::end_redirect(&mut redirect); }

            // wait for user to confirm to continue
            format::pair_progress(redirecting, i, total_pairs_rendering);
            io_redirect::confirm_continue();

            // resume redirecting stdout
            if redirecting {
                match out_file {
                    Some(p) => io_redirect::resume_redirect(&mut redirect, p),
                    None => {
                        panic!("set to redirect, but no out file found");
                    },
                };
            }
        }

        // retrieve names of both submissions
        let sub_a_name = sub_name(pair.a, mode);
        let sub_b_name = sub_name(pair.b, mode);

        // render header & table for this pair
        format::pair_header(
            redirecting, 
            i + 1, 
            &sub_a_name, 
            &sub_b_name, 
            pair.matches.len(),
            pair.percentile);
        pair_table(pair, (&sub_a_name, &sub_b_name), mode).printstd();
    }
}

// Wrappers for printing messages in result rendering, because 
// formatting can complicate things
mod format {
    use ansi_term::Colour::{RGB, White};
    use ansi_term::Style;

    // conditionally format a string with whatever formatting is supplied,
    // depending on whether or not output is being redirected
    macro_rules! cond_fmt {
        ($redirect:expr, $s:expr, $formatted:expr) => {
            if !$redirect {
                $formatted
            } else {
                Style::default().paint($s)
            }
        }
    }

    // print a message indicating that no overlap between submission was found
    pub fn no_overlap_msg(redir: bool) {
        let message = "Aye, no overlap was found!";

        let formatted = cond_fmt!(redir, message, 
            RGB(102, 224, 255).bold().paint(message));

        println!("\n{}", formatted);
    }

    // print a message indicating overlap was found
    pub fn overlap_found_msg(redir: bool) {
        let message = "Avast ye, there be submission overlap!";

        let formatted = cond_fmt!(redir, message, 
            RGB(77, 255, 77).bold().paint(message));

        println!("\n{}", formatted);
    }

    // print a message indicating how many submission pairs will be rendered
    pub fn num_pairs_rendering(redir: bool, thresh: f64, total: usize, total_render: usize) {
        if thresh > 0.0 {
            println!("Rendering pairs at least {}% of max matches: {} kept out of {} total", 
                thresh * 100.0, total_render, total);
        } else {
            println!("Rendering all submission pairs ({} total)", total_render);
        }
    }

    // print the header indicating pair number, pair names, & number of matches
    pub fn pair_header(redir: bool, n: usize, a_name: &String, b_name: &String, matches: usize,
        perc_of_max: f64) {
        let match_str = &format!("{} matches", matches);

        let match_fmt = cond_fmt!(redir, match_str, 
            RGB(77, 255, 77).bold().paint(match_str));
        let a_fmt = cond_fmt!(redir, a_name, 
            White.bold().paint(a_name));
        let b_fmt = cond_fmt!(redir, b_name, 
            White.bold().paint(b_name));

        println!("\nPair {}: {} and {}: {} ({:.2}% of max)", n, a_fmt, b_fmt, match_fmt, perc_of_max * 100.0);
    }

    // print a message indicating how many pairs have been rendered so far
    pub fn pair_progress(redirecting: bool, so_far: usize, total: usize) {
        let message = format!("Pausing at {} / {} pairs rendered.", so_far, total);

        let formatted = RGB(255, 255, 77).bold().paint(message);

        println!("\n{}", formatted);
    }
}

// Extract a "name" for a submission (for use in output) based on the sub mode: 
// - single-file: subs are named by their only document's filename
// - multi-file: subs are named by the dir that contains their document files
fn sub_name(sub: &Sub, mode: &SubFileMode) -> String {
    match mode {
        SubFileMode::Multi => {
            // retrieve the name of the bottom-most level dir from a pathbuf
            fn lowest_dir(p: &PathBuf) -> &str {
                let comp = p.components().last().unwrap();

                // extract the string inside
                if let Component::Normal(os_str) = comp {
                    os_str.to_str().unwrap()
                } else {
                    panic!("failed to retrieve dir component from path {}", p.display());
                }
            }

            // use multifile submission's dirname as its "name"
            format!("{}/", lowest_dir(sub.dir_name.as_ref().unwrap()))
        }
        SubFileMode::Single => {
            if sub.documents.is_empty() {
                panic!("submission with no documents: {:?}", sub);
            }

            // extract the filename from a path
            fn file_name(p: &PathBuf) -> String {
                String::from(p.file_name().unwrap().to_str().unwrap())
            }

            // extract file names of each submissions' singular doc
            match &sub.documents[0] {
                Doc::Processed(path, _) => file_name(path),
                _ => { panic!("unprocessed document encountered in {:?}", sub.documents[0]); },
            }
        }
    }
}

// Generate a table summarizing fingerprint matches for a given pair of submissions
fn pair_table(pair: &SubPair, names: (&String, &String), mode: &SubFileMode) -> Table {
    let mut t = Table::new();

    let (a_name, b_name) = names;
    let a_title = format!("{} ({:.2}%)", a_name, pair.a_percent * 100.0);
    let b_title = format!("{} ({:.2}%)", b_name, pair.b_percent * 100.0);

    // add title row: submission names & their content match percentages
    t.add_row(row!["", Fcbic->a_title, Fcbic->b_title]);

    let mut match_n = 1;    // match number (for leftmost column)

    // order hashes for predictability
    let mut ordered_hashes: Vec<i64> = pair.matches.iter().cloned().collect();
    ordered_hashes.sort();

    // for each shared hash
    for hash in ordered_hashes.iter() {
        // extract file/line number info for fingerprints with 
        // this hash in each submission
        let a_entry = format_line_numbers(&pair.a, *hash, mode).join("\n");
        let b_entry = format_line_numbers(&pair.b, *hash, mode).join("\n");

        // add a row for this match
        t.add_row(row![bc->match_n, a_entry, b_entry]);

        match_n += 1;
    }

    return t;   // constructed table for this pair
}

// Generate a vector of strings describing the lines (/files if multi-file
// submission) on which the indicated fingerprint occurs
fn format_line_numbers(sub: &Sub, hash: i64, mode: &SubFileMode) -> Vec<String> {
    let mut formatted = Vec::new();

    for doc in sub.documents.iter() {
        if let Doc::Processed(path, fps) = doc {
            let mut doc_line = String::new();

            // write doc filename to doc line in multi-file mode
            if let SubFileMode::Multi = mode {
                let fname = path.file_name().unwrap().to_str().unwrap();
                doc_line.push_str(&format!("{} ", fname));
            }

            let matched_lines = get_lines(doc, hash);   // get line ranges associated with this hash
            if matched_lines.is_empty() { continue; }   // skip if nothing to write

            // depending on number of lines found, write 'lines' or 'line'
            let (start, end) = matched_lines[0];
            if matched_lines.len() > 1 || (end - start > 0) {
                doc_line.push_str("lines ");
            } else {
                doc_line.push_str("line ");
            }

            let len = matched_lines.len();  // cache number of matched lines

            // for each line range
            for (i, range) in matched_lines.iter().enumerate() {
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
            panic!("unprocessed document encountered in format_line_numbers: {:?}", doc);
        }
    }

    formatted
}

// Construct a list of line ranges of all fingerprints in this doc 
// that have the given hash, combining overlapping/consecutive ranges
fn get_lines(doc: &Doc, hash: i64) -> Vec<(i32, i32)> {
    // Insert a line range into a vector of line ranges, ensuring that
    // overlapping/consecutive ranges are coalesced into one 
    // (eg (1,4) & (3,5) -> (1,5) and (2,5) & (6,10) -> (2,10))
    // (assumes inserting into an *already coalesced* vector)
    fn coalesce_insert(lines: &mut Vec<(i32, i32)>, new: (i32, i32)) {
        // if empty, nothing to coalesce
        if lines.is_empty() {
            lines.push(new);
        } else {
            // check last el for need to coalesce
            let lst = lines.pop().unwrap();
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
        }
    }

    // extract document path/fingerprints
    if let Doc::Processed(path, fps) = doc {
        let mut lines = Vec::new();

        // add line ranges for fingerprints that match the given hash
        for fp in fps.iter() {
            if fp.hash == hash {
                coalesce_insert(&mut lines, fp.lines);
            }
        }

        return lines;
    } else {
        panic!("unprocessed Doc encountered in get_lines: {:?}", doc);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use crate::fingerprint::Fingerprint;
    use std::path::PathBuf;

    #[test]
    fn test_sub_name() {
        // multi-file submissions
        {
            let sub = Sub {
                dir_name: Some(PathBuf::from("all-subs/sub-abcd/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("all-subs/sub-abcd/main.arr"), vec![]),
                    Doc::Processed(PathBuf::from("all-subs/sub-abcd/tests.arr"), vec![])
                ]
            };
            let name = sub_name(&sub, &SubFileMode::Multi);
            let exp_name = String::from("sub-abcd/");
            assert_eq!(name, exp_name);
        }
        {
            let sub = Sub {
                dir_name: Some(PathBuf::from("all-subs/sub-xyz/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("all-subs/sub-xyz/main.arr"), vec![]),
                    Doc::Processed(PathBuf::from("all-subs/sub-xyz/tests.arr"), vec![])
                ]
            };
            let name = sub_name(&sub, &SubFileMode::Multi);
            let exp_name = String::from("sub-xyz/");
            assert_eq!(name, exp_name);
        }
        {
            let sub = Sub {
                dir_name: Some(PathBuf::from("all-subs/sub-lmn/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("all-subs/sub-lmn/main.arr"), vec![]),
                    Doc::Processed(PathBuf::from("all-subs/sub-lmn/tests.arr"), vec![])
                ]
            };

            let name = sub_name(&sub, &SubFileMode::Multi);
            let exp_name = String::from("sub-lmn/");
            assert_eq!(name, exp_name);
        }

        // single-file submissions
        {
            let sub = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from("all-subs/submissionA.arr"), vec![])
                ]
            };

            let name = sub_name(&sub, &SubFileMode::Single);
            let exp_name = String::from("submissionA.arr");
            assert_eq!(name, exp_name);
        }
        {
            let sub = Sub {
                dir_name: None,
                documents: vec![
                    Doc::Processed(PathBuf::from("~/Desktop/nested/dirs/all-subs/this-is-the-sub-name.arr"), vec![])
                ]
            };

            let name = sub_name(&sub, &SubFileMode::Single);
            let exp_name = String::from("this-is-the-sub-name.arr");
            assert_eq!(name, exp_name);
        }
    }

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
                a_percent: 0.45,
                b: &b,
                b_percent: 0.78,
                matches: matches,
                percentile: 0.55
            };

            let a_name = String::from("sub1/");
            let b_name = String::from("sub2/");

            let exp_table = table!(
                ["", Fcbic->"sub1/ (45.00%)", Fcbic->"sub2/ (78.00%)"],
                [bc->"1", "doc1.arr lines 10-15", "doc1.arr lines 5, 17-30"],   // fp 11
                [bc->"2", "doc1.arr lines 1-3, 6-10\ndoc2.arr lines 44-57", "doc1.arr lines 8-12"], // fp 17
                [bc->"3", "doc1.arr line 5\ndoc2.arr lines 25-30", "doc2.arr lines 8-10"] // fp 20
            );

            assert_eq!(pair_table(&sp, (&a_name, &b_name), &SubFileMode::Multi), exp_table);
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
                a_percent: 0.22,
                b: &b,
                b_percent: 0.31,
                matches: matches,
                percentile: 0.55
            };

            let a_name = String::from("sub1.arr");
            let b_name = String::from("sub2.arr");

            let exp_table = table!(
                ["", Fcbic->"sub1.arr (22.00%)", Fcbic->"sub2.arr (31.00%)"],
                [bc->"1", "line 5", "lines 38-42"],   // fp 12
                [bc->"2", "lines 30-31", "lines 30-31"], // fp 17
                [bc->"3", "lines 4-5, 11-22", "lines 17-29"] // fp 28
            );

            assert_eq!(pair_table(&sp, (&a_name, &b_name), &SubFileMode::Single), exp_table);
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