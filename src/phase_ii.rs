
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

// Consider pairs of submissions that overlap, associate them with
// the fingerprints they share, and order pairs by the quantity shared.
fn find_overlaps(hash_to_subs: FnvHashMap<i64, HashSet<&Sub>>, threshold: f64)
    -> Vec<SubPair> {

    // a map whose keys are pairs (sets of size 2) of subs and whose values are sets of hashes
    let mut pairs_to_hashes: HashMap<BTreeSet<&Sub>, HashSet<i64>> = HashMap::new();

    // the highest HashSet<i64> size value in the pairs_to_hashes map, to be updated
    let mut max_num_hashes: usize = 0;

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
                    let mut sub_btset: BTreeSet<&Sub> = BTreeSet::new();
                    sub_btset.insert(*ordered_subs[i]);
                    sub_btset.insert(*ordered_subs[j]);

                        // update max_num_hashes if the size of the current value set is
                        // larger than the current value
                        let mut num_hashes: usize = 0;
                        let cur_val: Option<&HashSet<i64>> = pairs_to_hashes.get(&sub_btset);
                        match cur_val {
                            None => {}
                            Some(set) => { num_hashes = set.len(); }
                        }

                        if num_hashes > max_num_hashes {
                            max_num_hashes = num_hashes;
                        }

                        // if sub_pair is not already a key in pairs_to_hashes, add it and
                        // make it map to a set containing hash; otherwise, add hash to the
                        // set sub_pair already maps to
                        pairs_to_hashes.entry(sub_btset)
                            .or_insert_with(HashSet::new)
                            .insert(hash);
                    j += 1;
                }
                i += 1;
            }
        }
    }

    // iterate through pairs_to_hashes, add a SubPair corresponding to each key-value pair
    // to subpairs
    let mut subpairs: Vec<SubPair> = Vec::new();

    for (sub_btset, matching_hashes) in pairs_to_hashes {
        let mut sub_btset_iter = sub_btset.iter();
        let num_hashes: usize = matching_hashes.len();

        let sp: SubPair = SubPair {
            a: sub_btset_iter.next().unwrap(),
            a_percent: 0.0,
            b: sub_btset_iter.next().unwrap(),
            b_percent: 0.0,
            matches: matching_hashes,
            percentile: (num_hashes / max_num_hashes) as f64
        };

        subpairs.push(sp);
    }

    // sort the pair_hash_tuples vec by descending number of matches
    subpairs.sort_by(|a, b| a.matches.len().cmp(&b.matches.len()));

    // return the populated, sorted output
    subpairs
}
