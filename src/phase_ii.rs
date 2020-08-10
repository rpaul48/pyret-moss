
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use fnv::FnvHashMap;
use std::iter::FromIterator;
use crate::{Doc, Sub};

// Consider pairs of submissions that overlap, associate them with
// the fingerprints they share, and order pairs by the quantity shared.
fn find_overlaps(hash_to_subs: FnvHashMap<i64, HashSet<&Sub>>)
    -> Vec<(BTreeSet<&Sub>, HashSet<i64>)> {

        // a map whose keys are pairs (sets of size 2) of subs and whose values are sets of hashes
        let mut pairs_to_hashes: HashMap<BTreeSet<&Sub>, HashSet<i64>> = HashMap::new();

        // iterate through hash_to_subs by key
        for (hash, subs) in hash_to_subs {
            // if the current key has a value containing more than one entry
            let subs_len: usize = subs.len();
            if subs_len > 1 {

                // get all possible pairs of submissions within subs
                let ordered_subs: Vec<&&Sub> = Vec::from_iter(subs.iter());
                let mut i: usize = 0;

                while i < (subs_len - 1) {
                    let mut j: usize = i + 1;
                    while j < (subs_len) {
                        // the current pair of submissions
                        let mut sub_pair: BTreeSet<&Sub> = BTreeSet::new();
                        sub_pair.insert(*ordered_subs[i]);
                        sub_pair.insert(*ordered_subs[j]);

                            // if sub_pair is not already a key in pairs_to_hashes, add it and
                            // make it map to a set containing hash; otherwise, add hash to the
                            // set sub_pair already maps to
                            pairs_to_hashes.entry(sub_pair)
                                .or_insert_with(HashSet::new)
                                .insert(hash);
                        j += 1;
                    }
                    i += 1;
                }
            }
        }

        // iterate through pairs_to_hashes, add (key, value) tuples to the pair_hash_tuples Vec
        let mut pair_hash_tuples: Vec<(BTreeSet<&Sub>, HashSet<i64>)> = Vec::new();

        for (sub_pair, matching_hashes) in pairs_to_hashes {
            pair_hash_tuples.push((sub_pair, matching_hashes));
        }

        // sort the pair_hash_tuples vec by descending number of matches
        pair_hash_tuples.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        // return the populated, sorted output
        pair_hash_tuples
}
