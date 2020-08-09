
use std::collections::HashSet;
use fnv::FnvHashMap;
use crate::{Doc, Sub};

// Consider pairs of submissions that overlap, associate them with
// the fingerprints they share, and order pairs by the quantity shared.
fn find_overlaps(hash_to_subs: FnvHashMap<i64, HashSet<&Sub>>)
    -> Vec<(HashSet<&Sub>, HashSet<i64>)> {

        // let pairs_to_hashes: HashMap<HashSet<&Sub>, HashSet<i64>> = HashMap::new();


        // iterate through hash_to_subs by key

            // if the current key K has a value V with len > 1

                // for each pair P of subs in V, add K to the HashSet<i64> that P
                // maps to in pairs_to_hashes


        // let sorted_kv_tuples: Vec<(HashSet<&Sub>, HashSet<i64>)> = Vec::new();

        // iterate through pairs_to_hashes by key

            // insert the current KV pair into sorted_kv_tuples such that the Vec is always in
            // order of descending value length


        // sorted_kv_tuples

    unimplemented!();
}
