/* results.rs: Render findings of overlap between submissions, if any */

// Given a map from sub pairs to fingerprint hashes shared between them,
// ordered by amount of overlap, render a message to the user (to stdout or
// the given file) summarizing the overlaps
fn render_results(pairs_to_hashes: Vec<(HashSet<&Sub>, HashSet<i64>)>,
    out_file: Option<&Path>) {
    unimplemented!();
}