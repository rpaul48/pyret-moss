
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use fnv::FnvHashMap;
use std::iter::FromIterator;
use crate::{Doc, Sub};

// A SubPair represents an unordered pair of Subs with overlapping hashes, where
// each element X in the pair has a "percent" value, which is equal to the quotient of the size
// of matches and the number of fingerprints contained in X;
// the percentile value denotes how the size of matches compares to the sizes of other match sets
#[derive(Debug, Clone)]
pub struct SubPair<'a> {
    pub a: &'a Sub,
    pub a_percent: f64,
    pub b: &'a Sub,
    pub b_percent: f64,
    pub matches: HashSet<i64>,
    pub percentile: f64
}

// two SubPairs are equal if they each contain references to the same two Subs
impl PartialEq for SubPair<'_> {
    fn eq(&self, other: &Self) -> bool {
        ((self.a == other.a) && (self.b == other.b)) ||
        ((self.a == other.b) && (self.b == other.a))
    }
}

impl Eq for SubPair<'_> {}

// Consider pairs of submissions that overlap, associate them with the
// fingerprints they share, calculate 'percent' values for each Sub in
// a Pair and a 'percentile' value for each SubPair, keep pairs with percentile
// greater than input threshold, order pairs by the quantity shared and return in tuple
// along with number of total subpairs found
fn find_overlaps<'a>(hash_to_subs: &'a FnvHashMap<i64, HashSet<&Sub>>, threshold: f64)
    -> (Vec<SubPair<'a>>, usize) {

    // ensure 0 <= threshold <= 1
    if (threshold < 0.0) || (threshold > 1.0) {
        err!("The input percentile threshold must be between 0 and 1 inclusive.");
    }

    // a map whose keys are pairs (sets of size 2) of subs and whose values are sets of hashes
    let mut pairs_to_hashes: HashMap<BTreeSet<&Sub>, HashSet<i64>> = HashMap::new();

    // the highest HashSet<i64> size value in the pairs_to_hashes map, to be updated
    let mut max_num_hashes: usize = 0;

    // iterate through hash_to_subs by key
    for (hash, subs) in hash_to_subs {
        // if the current key has a value containing more than one entry, make pairs
        let subs_len: usize = subs.len();
        if subs_len > 1 {
            // get all possible pairs of submissions within subs
            let ordered_subs: Vec<&&Sub> = Vec::from_iter(subs.iter());
            let mut i: usize = 0;

            while i < (subs_len - 1) {
                let mut j: usize = i + 1;
                while j < (subs_len) {
                    // the current pair of submissions, represented as an unordered set
                    let mut sub_btset: BTreeSet<&Sub> = BTreeSet::new();
                    sub_btset.insert(*ordered_subs[i]);
                    sub_btset.insert(*ordered_subs[j]);

                        // retrieve the size of the current value set, account for the
                        // current element, which may or may not be added
                        let mut num_hashes: usize = 0;
                        let cur_val: Option<&HashSet<i64>> = pairs_to_hashes.get(&sub_btset);
                        match cur_val {
                            None => { num_hashes += 1; }
                            Some(set) => {
                                if set.contains(&hash) {
                                    // the hash wont be added to the set
                                    num_hashes = set.len();
                                } else {
                                    // the hash will be added to the set
                                    num_hashes = set.len() + 1;
                                }
                            }
                        }

                        // update max_num_hashes if the size of the current value set is
                        // larger than the current value
                        if num_hashes > max_num_hashes {
                            max_num_hashes = num_hashes;
                        }

                        // if sub_pair is not already a key in pairs_to_hashes, add it and
                        // make it map to a set containing hash; otherwise, add hash to the
                        // set sub_pair already maps to
                        pairs_to_hashes.entry(sub_btset)
                            .or_insert_with(HashSet::new)
                            .insert(*hash);
                    j += 1;
                }
                i += 1;
            }
            i += 1;
        }
    }

    // iterate through pairs_to_hashes, add a SubPair corresponding to each key-value pair
    // to the subpairs Vec, which will eventually be returned as output
    let mut subpairs: Vec<SubPair> = Vec::new();

        // iterate through pairs_to_hashes, add a SubPair corresponding to each key-value pair
        // to the subpairs Vec, which will eventually be returned as output with numallpairs
        let mut subpairs: Vec<SubPair> = Vec::new();
        let numallpairs: usize = pairs_to_hashes.len();

        for (sub_btset, matching_hashes) in pairs_to_hashes {
            let mut sub_btset_iter = sub_btset.iter();
            let num_hashes: usize = matching_hashes.len();

        // the two subs in the subpairs
        let sub_a: &Sub = sub_btset_iter.next().unwrap();
        let sub_b: &Sub = sub_btset_iter.next().unwrap();

        // the set of unique fingerprint hash values in the Docs of sub_a
        let mut all_fp_hashes_a: HashSet<i64> = HashSet::new();
        for doc in &sub_a.documents {
            match doc {
                Doc::Unprocessed(pathbuf) => {
                    err!("An Unprocessed Doc was found in {:?}", pathbuf);
                    }
                Doc::Processed(_pathbuf, fingerprints) => {
                    for fp in fingerprints {
                        all_fp_hashes_a.insert(fp.hash);
                    }
                }
            }
        }

        // the number of unique fingerprint hash values in the Docs of sub_b
        let mut all_fp_hashes_b: HashSet<i64> = HashSet::new();
        for doc in &sub_b.documents {
            match doc {
                Doc::Unprocessed(pathbuf) => {
                    err!("An Unprocessed Doc was found in {:?}", pathbuf);
                }
                Doc::Processed(_pathbuf, fingerprints) => {
                    for fp in fingerprints {
                        all_fp_hashes_b.insert(fp.hash);
                    }
                }
            }
        }

        // the SubPair representing the current pair of subs, to be added to the output
        let percentile: f64 = (num_hashes as f64) / (max_num_hashes as f64);

        // only add the SubPair if its percentile >= threshold
        if percentile >= threshold {
            let sp: SubPair = SubPair {
                a: sub_a,
                a_percent: (num_hashes as f64) / (all_fp_hashes_a.len() as f64),
                b: sub_b,
                b_percent: (num_hashes as f64) / (all_fp_hashes_b.len() as f64),
                matches: matching_hashes,
                percentile: percentile
            };

            subpairs.push(sp);
        }
    }

    // sort the pair_hash_tuples vec by descending percentile (same as sort by num of matches)
    subpairs.sort_by(|a, b| b.percentile.partial_cmp(&a.percentile).unwrap());

        // return the populated, sorted output
        (subpairs, numallpairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::Fingerprint;
    use crate::Doc::Processed;
    use crate::phase_i::analyze_subs;
    use std::io;
    use std::path::PathBuf;

    #[test]
    // tests expected output on two single-doc input subs in input FnvHashMap, namely that the
    // matched hashes are correctly recorded, and the percent values/percentile are correct
    fn test_single_pair_input() -> io::Result<()> {
        // original submissions
        let mut sub1 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file/sub1.arr"))
            ]
        };
        let mut sub2 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file/sub2.arr"))
            ]
        };

        let mut submissions = vec![&mut sub1, &mut sub2];
        let inp_map = analyze_subs(&mut submissions, None, 10, 60)?;
        let out = find_overlaps(&inp_map, 0.0);

        let mut exp_matches = HashSet::new();
        exp_matches.insert(5421077);
        exp_matches.insert(73943364);
        exp_matches.insert(14933625);

        let processed_sub1 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Processed(PathBuf::from("test-dirs/test/single-file/sub1.arr"), vec![
                    Fingerprint { hash: 5421077, lines: (11, 12) },
                    Fingerprint { hash: 31722361, lines: (15, 16) },
                    Fingerprint { hash: 30182096, lines: (16, 16) },
                    Fingerprint { hash: 14933625, lines: (17, 18) },
                    Fingerprint { hash: 73943364, lines: (19, 19) }])]
        };

        let processed_sub2 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Processed(PathBuf::from("test-dirs/test/single-file/sub2.arr"), vec![
                    Fingerprint { hash: 5421077, lines: (8, 10) },
                    Fingerprint { hash: 14933625, lines: (13, 14) },
                    Fingerprint { hash: 73943364, lines: (15, 15) }])]
        };

        let exp_out_sp = SubPair {
            a: &processed_sub1,
            a_percent: 0.6,
            b: &processed_sub2,
            b_percent: 1.0,
            matches: exp_matches,
            percentile: 1.0
        };

        assert_eq!(out, (vec![exp_out_sp], 1));
        Ok(())
    }

    #[test]
    // tests expected output on four single-doc input subs in input FnvHashMap, namely that
    // SubPairs are sorted by number of matches in output, percent/percentile values are correct,
    // and pairs without overlap are omitted
    fn test_multiple_pairs_output() -> io::Result<()> {
        // original submissions
        let mut sub1 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file-subpairs/sub1.arr"))
            ]
        };
        let mut sub2 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file-subpairs/sub2.arr"))
            ]
        };
        let mut sub3 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file-subpairs/sub3.arr"))
            ]
        };
        let mut sub4 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/single-file-subpairs/sub4.arr"))
            ]
        };

        let mut submissions = vec![&mut sub1, &mut sub2, &mut sub3, &mut sub4];
        let inp_map = analyze_subs(&mut submissions, None, 10, 60)?;
        let out_min_thresh = find_overlaps(&inp_map, 0.0);

        let processed_sub1 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Processed(PathBuf::from("test-dirs/test/single-file-subpairs/sub1.arr"), vec![
                    Fingerprint { hash: 5421077, lines: (11, 12) },
                    Fingerprint { hash: 31722361, lines: (15, 16) },
                    Fingerprint { hash: 30182096, lines: (16, 16) },
                    Fingerprint { hash: 14933625, lines: (17, 18) },
                    Fingerprint { hash: 73943364, lines: (19, 19) },
                    Fingerprint { hash: 21898048, lines: (22, 23) }])]
        };

        let processed_sub2 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Processed(PathBuf::from("test-dirs/test/single-file-subpairs/sub2.arr"), vec![
                    Fingerprint { hash: 5421077, lines: (8, 10) },
                    Fingerprint { hash: 14933625, lines: (13, 14) },
                    Fingerprint { hash: 73943364, lines: (15, 15) }])]
        };

        let processed_sub4 = Sub {
            dir_name: None,
            documents: vec![
                Doc::Processed(PathBuf::from("test-dirs/test/single-file-subpairs/sub4.arr"), vec![
                    Fingerprint { hash: 5421353, lines: (5, 6) },
                    Fingerprint { hash: 10580184, lines: (9, 10)},
                    Fingerprint { hash: 14933625, lines: (11, 12) },
                    Fingerprint { hash: 17304907, lines: (13, 14) },
                    Fingerprint { hash: 21898048, lines: (17, 18) }])]
        };

        let mut sub1_sub2_matches = HashSet::new();
        sub1_sub2_matches.insert(73943364);
        sub1_sub2_matches.insert(5421077);
        sub1_sub2_matches.insert(14933625);

        let mut sub1_sub4_matches = HashSet::new();
        sub1_sub4_matches.insert(21898048);
        sub1_sub4_matches.insert(14933625);

        let mut sub2_sub4_matches = HashSet::new();
        sub2_sub4_matches.insert(14933625);

        let sub1_sub2_pair = SubPair {
            a: &processed_sub1,
            a_percent: 0.5,
            b: &processed_sub2,
            b_percent: 1.0,
            matches: sub1_sub2_matches,
            percentile: 1.0
        };

        let sub1_sub4_pair = SubPair {
            a: &processed_sub1,
            a_percent: 1.0 / 3.0,
            b: &processed_sub4,
            b_percent: 0.4,
            matches: sub1_sub4_matches,
            percentile: 2.0 / 3.0
        };

        let sub2_sub4_pair = SubPair {
            a: &processed_sub2,
            a_percent: 1.0 / 3.0,
            b: &processed_sub4,
            b_percent: 0.2,
            matches: sub2_sub4_matches,
            percentile: 1.0 / 3.0
        };

        assert_eq!(out_min_thresh, (vec![sub1_sub2_pair, sub1_sub4_pair, sub2_sub4_pair], 3));

        Ok(())
    }

    #[test]
    // tests expected output on four multi-doc input subs in input FnvHashMap, namely that
    // SubPairs are sorted by number of matches in output, percent/percentile values are correct,
    // and pairs below percentile threshold are omitted
    fn test_multiple_multidoc_pairs_output() -> io::Result<()> {
        // original submissions
        let mut sub1 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file-subpairs/sub1")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub1/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub1/main.arr"))
            ]
        };
        let mut sub2 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file-subpairs/sub2")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub2/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub2/main.arr"))
            ]
        };
        let mut sub3 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file-subpairs/sub3")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub3/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub3/main.arr"))
            ]
        };
        let mut sub4 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file-subpairs/sub4")),
            documents: vec![
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub4/common.arr")),
                Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub4/main.arr"))
            ]
        };

        let mut submissions = vec![&mut sub1, &mut sub2, &mut sub3, &mut sub4];
        let inp_map = analyze_subs(&mut submissions, None, 10, 60)?;
        //threshold is such that some pairs are filtered out
        let out_med_thresh = find_overlaps(&inp_map, 0.3);

        let processed_sub1 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file-subpairs/sub1")),
            documents: vec![
                Processed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub1/common.arr"),
                    vec![Fingerprint { hash: 390399223, lines: (1, 2) }]),
                Processed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub1/main.arr"), vec![
                    Fingerprint { hash: 103309548, lines: (3, 5) },
                	Fingerprint { hash: 139046768, lines: (7, 8) },
                	Fingerprint { hash: 157553660, lines: (12, 12) },
                	Fingerprint { hash: 155828129, lines: (16, 17) },
                	Fingerprint { hash: 70845857, lines: (17, 18) }])]
        };

        let processed_sub3 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file-subpairs/sub3")),
            documents: vec![
                Processed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub3/common.arr"),
                vec![Fingerprint { hash: 76905376, lines: (6, 7) },
                    Fingerprint { hash: 76839850, lines: (7, 8) },
                    Fingerprint { hash: 41033526, lines: (8, 8) },
                    Fingerprint { hash: 77033123, lines: (8, 9) },
                    Fingerprint { hash: 70845857, lines: (16, 17) }]),
                Processed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub3/main.arr"), vec![
                    Fingerprint { hash: 103309548, lines: (3, 5) },
                    Fingerprint { hash: 103309548, lines: (13, 15) },
                    Fingerprint { hash: 138677810, lines: (22, 25) },
                    Fingerprint { hash: 90448699, lines: (26, 26) },
                    Fingerprint { hash: 90391867, lines: (26, 27) },
                    Fingerprint { hash: 40051188, lines: (27, 27) },
                    Fingerprint { hash: 1866481, lines: (27, 27) }])]
        };

        let processed_sub4 = Sub {
            dir_name: Some(PathBuf::from("test-dirs/test/multi-file-subpairs/sub4")),
            documents: vec![
                Processed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub4/common.arr"),
                vec![Fingerprint { hash: 84319302, lines: (14, 14) },
                    Fingerprint { hash: 83117630, lines: (14, 14) },
                    Fingerprint { hash: 77155669, lines: (14, 14) },
                    Fingerprint { hash: 76905376, lines: (14, 15) },
                    Fingerprint { hash: 76839850, lines: (15, 16) },
                    Fingerprint { hash: 41033526, lines: (16, 16) },
                    Fingerprint { hash: 40051188, lines: (20, 20) },
                    Fingerprint { hash: 1866481, lines: (20, 20) }]),
                Processed(PathBuf::from("test-dirs/test/multi-file-subpairs/sub4/main.arr"), vec![
                    Fingerprint { hash: 103309548, lines: (4, 6) }])]
        };

        let mut sub3_sub4_matches = HashSet::new();
        sub3_sub4_matches.insert(103309548);
        sub3_sub4_matches.insert(76905376);
        sub3_sub4_matches.insert(1866481);
        sub3_sub4_matches.insert(41033526);
        sub3_sub4_matches.insert(76839850);
        sub3_sub4_matches.insert(40051188);

        let mut sub1_sub3_matches = HashSet::new();
        sub1_sub3_matches.insert(70845857);
        sub1_sub3_matches.insert(103309548);

        let sub3_sub4_pair = SubPair {
            a: &processed_sub3,
            a_percent: 6.0 / 11.0,
            b: &processed_sub4,
            b_percent: 2.0 / 3.0,
            matches: sub3_sub4_matches,
            percentile: 1.0
        };

        let sub1_sub3_pair = SubPair {
            a: &processed_sub1,
            a_percent: 2.0 / 3.0,
            b: &processed_sub3,
            b_percent: 2.0 / 11.0,
            matches: sub1_sub3_matches,
            percentile: 1.0 / 3.0
        };

        assert_eq!(out_med_thresh, (vec![sub3_sub4_pair, sub1_sub3_pair], 6));
        Ok(())
    }

}
