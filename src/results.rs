/* results.rs: Render findings of overlap between submissions, if any */

use cli::SubFileMode;

// Given a map from sub pairs to fingerprint hashes shared between them,
// ordered by amount of overlap, render a message to the user (to stdout or
// the given file) summarizing the overlaps
fn render_results(pairs_to_hashes: Vec<(HashSet<&Sub>, HashSet<i64>)>, out_file: Option<&Path>,
    mode: SubFileMode) {
    unimplemented!();
    /*
        if Some out_file: redirect println! temporarily

        if pairs_to_hashes is empty
            log "Aye, no overlap was found" & exit

        log "Avast ye, there be submission overlap!"

        for each (sub1, sub2) in pairs_to_hashes
            render_pair(sub1, sub2, fp_hashes)
    */
}

// Render summary of matches for a given pair of submissions, using a table format
fn render_pair(sub1: &Sub, sub2: &Sub, fp_hashes: &HashSet<i64>) {
    unimplemented!();
    /*
        log "\nsub1 and sub2: {} match{}", fp_hashes.len(), "es" or ""

        let sub1_matched, sub2_matched = compute percent_matched for each
        add title row to table (if single file mode, use file name as submission name)

        let match_n = 1;

        for hash in fp_hashes:
            sub1_entry = format_lines(sub1, hash)
            sub2_entry = format_lines(sub2, hash)

            add row![match_n, sub1_entry, sub2_entry]
            match_n++
        
        print table
    */
}

// Calculate what percentage of lines in the given submission match
// the given fingerprints
fn percent_matched(sub: &Sub, fp_hashes: &HashSet<i64>) -> f64 {
    unimplemented!();
    /*
        total_lines = 0
        lines_matched = 0

        for doc in sub.documents:
            total_lines += doc's lines
            covered_ranges = []

            for hash in fp_hashes:
                for line_range in doc.get_lines(hash):
                    unify(line_range, covered_ranges)
            
            for range in covered_ranges:
                lines_matched += max - min + 1
        
        return lines_matched / total_lines
    */
    /*
        fn unify(range: (i32, i32), ranges: &mut Vec<(i32, i32)>) {
            add range to ranges, or if overlaps with another range, unify them into one
        }
    */
}

// Generate a formatted string describing the lines (/files if multi-file
// submission) on which the indicated fingerprint occurs
fn format_lines(sub: &Sub, hash: i64, mode: SubFileMode) -> String {
    unimplemented!();
    /*
        out = ""

        for each doc in sub.documents:
            if multi-file mode:
                write filename to out
            
            lines = doc.get_lines(hash)

            if lines == 0: continue

            if lines > 1 or (==1 and end - start > 0)
                write "lines "
            else:
                write "line "
            
            for range in lines:
                if start - end == 0
                    write start
                else:
                    write "{}-{}", start, end
            write "\n"
        
        return out
    */
}



// sub1/ and sub2/: 3 matches
// +---+---------------------------+----------------------+
// |   |        sub1/ (67%)        |     sub2/ (24%)      |
// +---+---------------------------+----------------------+
// | 1 | doc1.arr lines 3-6, 18-21 | doc1.arr lines 39-44 |
// |   | doc2.arr lines 10-12      |                      |
// +---+---------------------------+----------------------+
// | 2 | doc1.arr lines 30-34      | doc1.arr lines 14-26 |
// +---+---------------------------+----------------------+
// | 3 | doc4.arr line 6           | doc3.arr lines 20-21 |
// |   | doc2.arr lines 1-7        | doc2.arr line 1      |
// |   |                           | doc4.arr lines 34-39 |
// +---+---------------------------+----------------------+

// submission1.arr and submission2.arr: 3 matches
// +---+-----------------------+-----------------------+
// |   | submission1.arr (67%) | submission2.arr (24%) |
// +---+-----------------------+-----------------------+
// | 1 | lines 3-6, 18-21      | lines 39-44           |
// +---+-----------------------+-----------------------+
// | 2 | lines 30-34           | lines 14-26           |
// +---+-----------------------+-----------------------+
// | 3 | line 1-7              | lines 20-21, 34-39    |
// +---+-----------------------+-----------------------+
