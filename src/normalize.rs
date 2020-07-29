/* normalize.rs: Pre-processer for Pyret programs to eliminate irrelevant features */

// A LineMapping encodes line number information from the original
// file from which normalized text has been generated.
// 
// line_ends[x] = y means that y is the index of the first char
// in the normalized text occurring *after* line x+1 in the original
#[derive(Debug, PartialEq)]
pub struct LineMapping {
    line_ends: Vec<i32>
}

impl LineMapping {
    // determine the line number in the original text that
    // a char at index i in the normalized text corresponds to
    fn line_number(i: i32) -> i32 {
        unimplemented!();
    }
}

// Remove/normalize any features from a program's text that 
// shouldn't differentiate it from other programs:
//      1. normalize whitespace to a single space
//      2. normalize identifiers
//      3. remove type annotations
//      4. remove docstrings
//      5. remove comments
// Returns the normalized string & enough info to map parts 
// of the normalized text to line numbers in the original (LineMapping)
pub fn normalize(program: String) -> (String, Box<LineMapping>) {
    unimplemented!();
}


#[cfg(test)]
mod normalize_tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn whitespace_normalized() {
        {
            let (norm, lm) = normalize(String::from("  \n \na = 1\n\t\t "));
            let out = String::from(" v = 1 ");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![1, 1, 6, 7] });
        }
        {
            let (norm, lm) = normalize(String::from("check:\n\n\t1 is \n2\nend"));
            let out = String::from("check: 1 is 2 end");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![6, 6, 11, 13, 16] });
        }
    }

    #[test]
    fn identifiers_normalized() {
        {
            let (norm, lm) = normalize(
                String::from("name-1 = 7\nsecond_name = name-1 * name-1"));
            let out = String::from("v = 7 v = v * v");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![6, 15] });
        }
    }

    #[test]
    fn types_removed() {
        {
            let (norm, lm) = normalize(
                String::from("x :: Number = 10\ny :: Boolean = true"));
            let out = String::from("v = 10 v = true");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![7, 15] });
        }
        {
            let (norm, lm) = normalize(
                String::from("fun f(a :: Custom, b :: List)\n-> String:\n"));
            let out = String::from("fun v(v, v):");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![11, 12, 12] });
        }
        // vary whitespace with -> output annotations
        // use complex types
    }

    #[test]
    fn docs_removed() {
        {
            let (norm, lm) = normalize(
                String::from("fun f():\n\tdoc: \"docstring here\"\n5\nend"));
            let out = String::from("fun v(): 5 end");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![9, 9, 11, 14] });
        }
        // try ``` strings on multi-lines here too
    }

    #[test]
    fn comments_removed() {
        {
            let (norm, lm) = normalize(
                String::from("x = 1 # x is 1\ny = 2 # the value of y"));
            let out = String::from("v = 1 v = 2 ");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![6, 12] });
        }
        // multiline comments
    }

    #[test]
    fn single_line() {
        let (norm, lm) = normalize(String::from("fun square(n): n * n end"));
        let out = String::from("fun v(v): v * v end");

        assert_eq!(norm, out);
        assert_eq!(*lm, LineMapping { line_ends: vec![out.len().try_into().unwrap()] });
    }
}