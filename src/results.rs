/* results.rs: Render findings of overlap between submissions, if any */

use crate::{Sub, Doc};
use std::path::{Path, PathBuf, Component};
use crate::cli::SubFileMode;
use crate::phase_ii::SubPair;
use crate::io_redirect;
use crate::phase_iii::{self, Entry, Match};
use std::collections::HashSet;
use prettytable::Table;

// number of results to display before prompting the user to continue
const RESULT_BUFFER_SIZE: usize = 50;

// Given a vector of matched submission pairs ordered by amount of overlap, 
// render a message (to stdout or the given file) summarizing the overlaps
pub fn render_results(sub_dir: &Path, sub_pairs: Vec<SubPair>, mode: &SubFileMode, out_file: Option<&Path>, 
    match_thresh: f64, total_pairs: usize, no_pauses: bool, verbose: bool) {

    if verbose { 
        println!("\nRendering results...");
        if no_pauses { println!("Not pausing during output"); }
    }

    // if output filepath given, start redirecting stdout to that file
    let redirecting = out_file.is_some();
    let mut redirect = match out_file {
        Some(p) => {
            if verbose { println!("Redirecting output to {}", p.display()); }
            Some(io_redirect::initialize_redirect(p))
        },
        None => None,
    };

    // show a header message with the submissions dir path
    format::results_header(sub_dir);

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

        // analyze common substrings of fingerprints to get a vector of matches
        let matches = phase_iii::analyze_pair(pair);
        
        pair_table(pair, (&sub_a_name, &sub_b_name), matches, mode).printstd();
    }
}

// Wrappers for printing messages in result rendering, because 
// formatting can complicate things
mod format {
    use std::path::{Path, PathBuf};
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

    // print header for the results, containing the submission directory for later reference
    pub fn results_header(sub_dir: &Path) {
        // convert submission dir path to absolute path
        let sub_dir = if let Ok(full_path) = std::fs::canonicalize(sub_dir) {
            full_path
        } else {
            PathBuf::from(sub_dir)
        };

        println!("\nSubmissions Directory: {}", sub_dir.display());
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
    pub fn num_pairs_rendering(_redir: bool, thresh: f64, total: usize, total_render: usize) {
        if thresh > 0.0 {
            println!("Rendering pairs at least {:.2}% of max matches: {} kept / {} total", 
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
    pub fn pair_progress(_redir: bool, so_far: usize, total: usize) {
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
fn pair_table(pair: &SubPair, names: (&String, &String), matches: Vec<Match>, mode: &SubFileMode) -> Table {
    let mut table = Table::new();

    let (a_name, b_name) = names;
    let a_title = format!("{} ({:.2}%)", a_name, pair.a_percent * 100.0);
    let b_title = format!("{} ({:.2}%)", b_name, pair.b_percent * 100.0);

    // add title row: submission names & their content match percentages
    table.add_row(row!["(size)", Fcbic->a_title, Fcbic->b_title]);

    // add each match to table
    for m in matches.iter() {
        // generate text for sub A & B cells
        let a_cell = format_entries(&m.a_entries, pair.a, mode);
        let b_cell = format_entries(&m.b_entries, pair.b, mode);

        // add row with match size & entries from each submission
        table.add_row(row![bc->m.size, a_cell, b_cell]);
    }

    table   // constructed table for this pair
}

// Generate a string describing the given entries, for a single cell of a sub pair table
fn format_entries(entries: &HashSet<Entry>, sub: &Sub, mode: &SubFileMode) -> String {
    let mut entries: Vec<_> = entries.into_iter().collect();

    // first sort by line range beginnings, then sort by 
    // document index to group docs together for readability
    entries.sort_by(|a, b| a.lines.0.cmp(&b.lines.0));
    entries.sort_by(|a, b| a.doc_idx.cmp(&b.doc_idx));

    let mut entry_text = Vec::new();

    for entry in entries.iter() {
        let mut entry_line = String::new();

        // if multi-file mode
        if let SubFileMode::Multi = mode {
            let doc = &sub.documents[entry.doc_idx];

            // write document filename to line
            if let Doc::Processed(path, _) = doc {
                let fname = path.file_name().unwrap().to_str().unwrap();
                entry_line.push_str(&format!("{} ", fname));
            } else {
                panic!("invalid document encountered while formatting output: {:?}", doc);
            }
        }

        let (start, end) = entry.lines;

        // write line numbers
        if end - start > 0 {
            entry_line.push_str(&format!("lines {}-{}", start, end));
        } else {
            entry_line.push_str(&format!("line {}", start));
        }

        entry_text.push(entry_line);
    }

    entry_text.join("\n")
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

    // turn a vector into a hashset (convenience)
    fn set<T: Clone+Eq+std::hash::Hash>(elts: Vec<T>) -> HashSet<T> {
        elts.iter().cloned().collect()
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

            let matches = vec![
                // [17]
                Match {
                    size: 1,
                    a_entries: set(vec![
                        Entry { doc_idx: 0, lines: (1, 3) },
                        Entry { doc_idx: 0, lines: (6, 10) },
                        Entry { doc_idx: 1, lines: (44, 57) },
                    ]),
                    b_entries: set(vec![
                        Entry { doc_idx: 0, lines: (8, 12) }
                    ])
                },
                // [20]
                Match {
                    size: 1,
                    a_entries: set(vec![
                        Entry { doc_idx: 0, lines: (5, 5) },
                        Entry { doc_idx: 1, lines: (25, 30) },
                    ]),
                    b_entries: set(vec![
                        Entry { doc_idx: 1, lines: (8, 10) }
                    ])
                },
                // [11]
                Match {
                    size: 1,
                    a_entries: set(vec![
                        Entry { doc_idx: 0, lines: (10, 11) },
                        Entry { doc_idx: 0, lines: (12, 15) },
                    ]),
                    b_entries: set(vec![
                        Entry { doc_idx: 0, lines: (5, 5) },
                        Entry { doc_idx: 0, lines: (17, 30) }
                    ])
                }
            ];

            let exp_table = table!(
                ["(size)", Fcbic->"sub1/ (45.00%)", Fcbic->"sub2/ (78.00%)"],
                [bc->"1", "doc1.arr lines 1-3\ndoc1.arr lines 6-10\ndoc2.arr lines 44-57", "doc1.arr lines 8-12"],   // fp 17
                [bc->"1", "doc1.arr line 5\ndoc2.arr lines 25-30", "doc2.arr lines 8-10"], // fp 20
                [bc->"1", "doc1.arr lines 10-11\ndoc1.arr lines 12-15", "doc1.arr line 5\ndoc1.arr lines 17-30"] // fp 11
            );

            let out = pair_table(&sp, (&a_name, &b_name), matches, &SubFileMode::Multi);

            assert_eq!(out, exp_table);
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

            let matches = vec![
                // [28, 28, 17]
                Match {
                    size: 3,
                    a_entries: set(vec![
                        Entry { doc_idx: 0, lines: (16, 31) }
                    ]),
                    b_entries: set(vec![
                        Entry { doc_idx: 0, lines: (17, 31) }
                    ])
                },
                // [28. 28]
                Match {
                    size: 2,
                    a_entries: set(vec![
                        Entry { doc_idx: 0, lines: (11, 19) }
                    ]),
                    b_entries: set(vec![
                        Entry { doc_idx: 0, lines: (17, 29) }
                    ])
                },
                // [28]
                Match {
                    size: 1,
                    a_entries: set(vec![
                        Entry { doc_idx: 0, lines: (4, 5) }
                    ]),
                    b_entries: set(vec![
                        Entry { doc_idx: 0, lines: (17, 17) }
                    ])
                },
                // [12]
                Match {
                    size: 1,
                    a_entries: set(vec![
                        Entry { doc_idx: 0, lines: (5, 5) }
                    ]),
                    b_entries: set(vec![
                        Entry { doc_idx: 0, lines: (38, 42) }
                    ])
                }
            ];

            let exp_table = table!(
                ["(size)", Fcbic->"sub1.arr (22.00%)", Fcbic->"sub2.arr (31.00%)"],
                [bc->"3", "lines 16-31", "lines 17-31"],   // [28, 28, 17]
                [bc->"2", "lines 11-19", "lines 17-29"], // [28, 28]
                [bc->"1", "lines 4-5", "line 17"], // [28]
                [bc->"1", "line 5", "lines 38-42"] // [12]
            );

            let out = pair_table(&sp, (&a_name, &b_name), matches, &SubFileMode::Single);

            assert_eq!(out, exp_table);
        }
    }

    #[test]
    fn test_format_entries() {
        {
            let entries = set(vec![
                Entry { doc_idx: 2, lines: (15, 18) },
                Entry { doc_idx: 0, lines: (1, 4) },
                Entry { doc_idx: 2, lines: (2, 8) },
                Entry { doc_idx: 1, lines: (3, 20) }
            ]);

            let sub = Sub {
                dir_name: Some(PathBuf::from("~/submissions/sub/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("~/submissions/sub/one.arr"), Vec::new()),
                    Doc::Processed(PathBuf::from("~/submissions/sub/two.arr"), Vec::new()),
                    Doc::Processed(PathBuf::from("~/submissions/sub/three.arr"), Vec::new())
                ]
            };

            let exp_cell = String::from("one.arr lines 1-4\ntwo.arr lines 3-20\nthree.arr lines 2-8\nthree.arr lines 15-18");

            assert_eq!(format_entries(&entries, &sub, &SubFileMode::Multi), exp_cell);
        }
        {
            let entries = set(vec![
                Entry { doc_idx: 3, lines: (2, 5) },
                Entry { doc_idx: 0, lines: (1, 5) },
                Entry { doc_idx: 0, lines: (4, 8) }
            ]);
    
            let sub = Sub {
                dir_name: Some(PathBuf::from("dir/abcd/")),
                documents: vec![
                    Doc::Processed(PathBuf::from("dir/abcd/first.arr"), Vec::new()),
                    Doc::Processed(PathBuf::from("dir/abcd/second.arr"), Vec::new()),
                    Doc::Processed(PathBuf::from("dir/abcd/third.arr"), Vec::new()),
                    Doc::Processed(PathBuf::from("dir/abcd/fourth.arr"), Vec::new())
                ]
            };
    
            let exp_cell = String::from("first.arr lines 1-5\nfirst.arr lines 4-8\nfourth.arr lines 2-5");
    
            assert_eq!(format_entries(&entries, &sub, &SubFileMode::Multi), exp_cell);
        }
    }
}