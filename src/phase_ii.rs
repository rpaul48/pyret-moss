
use std::collections::HashSet;
use fnv::FnvHashMap;
use crate::{Doc, Sub};

// Consider pairs of submissions that overlap, associate them with
// the fingerprints they share, and order pairs by the quantity shared.
fn find_overlaps(hash_to_subs: FnvHashMap<i64, HashSet<&Sub>>) 
    -> Vec<(HashSet<&Sub>, HashSet<i64>)> {
    unimplemented!();
}