/* normalize.rs: Pre-processer for Pyret programs to eliminate irrelevant features */

use regex::Regex;

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

// determine if a string slice matches a Pyret keyword
fn is_pyret_keyword(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            "^(and|as|ascending|ask|by|cases|check|data|descending|do|does-not-raise\
                |else|else if|end|examples|extend|extract|false|for|from|fun|hiding|if\
                |import|include|is|is==|is=~|is-not|is-not==|is-not=~|is-not<=>|is-roughly\
                |is<=>|because|lam|lazy|let|letrec|load-table|method|module|newtype|of|or\
                |provide|provide-types|raises|raises-other-than|raises-satisfies\
                |raises-violates|reactor|rec|ref|sanitize|satisfies|select|shadow|sieve\
                |spy|order|transform|true|type|type-let|using|var|violates|when|block:\
                |check:|doc:|else:|examples:|otherwise:|provide:|row:|sharing:|source:\
                |table:|then:|where:|with:)$"
        ).unwrap();
    }
    RE.is_match(s)
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
        {
            let (norm, lm) = normalize(
                String::from("param :: List<List<InnerType>>"));
            let out = String::from("v");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![1] });
        }
        {
            let (norm, lm) = normalize(
                String::from("complex :: ((Number -> String) -> \
                            (List<String> -> List<List<Number>>)) = 10"));
            let out = String::from("v = 10");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![6] });
        }
    }

    #[test]
    fn docs_removed() {
        {
            let (norm, lm) = normalize(
                String::from("fun f():\n\
                                \tdoc: \"docstring here\"\n\
                                5\n\
                            end"));
            let out = String::from("fun v(): 5 end");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![9, 9, 11, 14] });
        }
        {
            let (norm, lm) = normalize(
                String::from("fun g():\n\
                                doc: ```This is a longer docstring.\n\
                                It takes place over multiple lines.```\n\
                                0\n\
                            end"));
            let out = String::from("fun v(): 0 end");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![9, 9, 9, 11, 14] });
        }
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
        {
            let (norm, lm) = normalize(
                String::from("n = true #| commented code:\n\
                                x = 15\n\
                                y =\"string value\"\n\
                                |#\n\
                            m = false"));
            let out = String::from("v = true v = false");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![9, 9, 9, 9, 18] });
        }
    }

    #[test]
    fn simple_func() {
        let (norm, lm) = normalize(String::from("fun square(n): n * n end"));
        let out = String::from("fun v(v): v * v end");
        assert_eq!(norm, out);
        assert_eq!(*lm, LineMapping { line_ends: vec![out.len().try_into().unwrap()] });
    }

    #[test]
    fn keywords_and_otherwise_preserved() {
        // ensure any other syntactic elements are preserved
        {
            let (norm, lm) = normalize(
                String::from("import tables as T"));
            let out = String::from("import v as v");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![13] });
        }
        {
            let (norm, lm) = normalize(
                String::from("if (5 * 2) < 10:\n\
                                    true\n\
                                else:\n\
                                    false\n\
                                end"));
            let out = String::from("if (5 * 2) < 10: true else: false end");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![17, 22, 28, 34, 37] });
        }
        {
            let (norm, lm) = normalize(
                String::from("examples:\n\
                                tmp = \"x = 5\"\n\
                                tmp is tmp\n\
                            end"));
            let out = String::from("examples: v = \"x = 5\" v is v end");
            assert_eq!(norm, out);
            assert_eq!(*lm, LineMapping { line_ends: vec![10, 22, 29, 32] });
        }
    }

    #[test]
    fn keyword_detection() {
        // yes, pyret
        assert!(is_pyret_keyword("check:"));
        assert!(is_pyret_keyword("is<=>"));
        assert!(is_pyret_keyword("var"));
        assert!(is_pyret_keyword("import"));
        assert!(is_pyret_keyword("lam"));
        assert!(is_pyret_keyword("raises-other-than"));

        // no, pyret
        assert!(!is_pyret_keyword("1"));
        assert!(!is_pyret_keyword("custom-identifier-name"));
        assert!(!is_pyret_keyword("and-or"));
        assert!(!is_pyret_keyword("ref-but-this-is-a-name"));
        assert!(!is_pyret_keyword("let-me-go"));
    }
}