/* normalize.rs: Pre-processer for Pyret programs to eliminate irrelevant features */

use regex::Regex;

// A NormText stores the normalized text of some program and
// encodes line number information from the original
// file from which normalized version has been generated.
// (accessible from line_number method)
//
// line_ends[x] = y means that y is the index of the first char
// in the normalized text occurring *after* line x+1 in the original
#[derive(Debug, PartialEq)]
pub struct NormText {
    pub value: String,
    line_ends: Vec<i32>
}

impl NormText {
    // determine the line number in the original text that
    // a char at index i in the normalized text corresponds to
    pub fn line_number(i: i32) -> i32 {
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
pub fn normalize(program: String) -> NormText {
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

    // generic testing form for normalize()
    // calls normalize() on input string & asserts output text value & line ends
    fn test_norm(input: &str, out_val: &str, out_line_ends: Vec<i32>) {
        let norm = normalize(String::from(input));
        assert_eq!(norm.value, String::from(out_val));
        assert_eq!(norm.line_ends, out_line_ends);
    }

    #[test]
    fn whitespace_normalized() {
        test_norm(
            "  \n \na = 1\n\t\t ",
            " v = 1 ",
            vec![1, 1, 6, 7]);
        test_norm(
            "check:\n\n\t1 is \n2\nend",
            "check: 1 is 2 end",
            vec![6, 6, 11, 13, 16]);
    }

    #[test]
    fn identifiers_normalized() {
        test_norm(
            "name-1 = 7\nsecond_name = name-1 * name-1",
            "v = 7 v = v * v",
            vec![6, 15]);
    }

    #[test]
    fn types_removed() {
        test_norm(
            "x :: Number = 10\ny :: Boolean = true",
            "v = 10 v = true",
            vec![7, 15]);
        test_norm(
            "fun f(a :: Custom, b :: List)\n-> String:\n",
            "fun v(v, v):",
            vec![11, 12, 12]);
        test_norm(
            "param :: List<List<InnerType>>",
            "v",
            vec![1]);
        test_norm(
            "complex :: ((Number -> String) -> \
            (List<String> -> List<List<Number>>)) = 10",

            "v = 10",
            vec![6]);
    }

    #[test]
    fn docs_removed() {
        test_norm(
            "fun f():\n\
                \tdoc: \"docstring here\"\n\
                5\n\
            end",

            "fun v(): 5 end",
            vec![9, 9, 11, 14]);

        test_norm(
            "fun g():\n\
                doc: ```This is a longer docstring.\n\
                It takes place over multiple lines.```\n\
                0\n\
            end",

            "fun v(): 0 end",
            vec![9, 9, 9, 11, 14]);
    }

    #[test]
    fn comments_removed() {
        test_norm(
            "x = 1 # x is 1\ny = 2 # the value of y",
            "v = 1 v = 2 ",
            vec![6, 12]);
        test_norm(
            "n = true #| commented code:\n\
            x = 15\n\
            y =\"string value\"\n\
            |#\n\
            m = false",

            "v = true v = false",
            vec![9, 9, 9, 9, 18]);
    }

    #[test]
    fn simple_func() {
        test_norm(
            "fun square(n): n * n end",
            "fun v(v): v * v end",
            vec![19]);
    }

    #[test]
    fn keywords_and_otherwise_preserved() {
        // ensure any other syntactic elements are preserved
        test_norm(
            "import tables as T",
            "import v as v",
            vec![13]);
        test_norm(
            "if (5 * 2) < 10:\n\
                true\n\
            else:\n\
                false\n\
            end",

            "if (5 * 2) < 10: true else: false end",
            vec![17, 22, 28, 34, 37]);
        test_norm(
            "examples:\n\
                tmp = \"x = 5\"\n\
                tmp is tmp\n\
            end",

            "examples: v = \"x = 5\" v is v end",
            vec![10, 22, 29, 32]);
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