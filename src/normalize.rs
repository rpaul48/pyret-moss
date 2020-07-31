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
pub fn normalize(program: &str) -> NormText {    
    let mut head: &str = &program;      // rest of program to be processed
    let mut norm = String::from("");    // normalized program text
    let mut norm_idx = 0;               // next index to write to in norm text
    let mut line_ends = Vec::new();     // encodes line info (see NormText above)

    // while haven't seen entire program
    while !head.is_empty() {

        // ------- Keywords & Identifiers -------
        if let Some((is_keyw, mat, rest, len)) = match_keyword_or_ident(head) {
            head = rest;    // jump over keyword/ident

            if is_keyw {
                norm.push_str(mat); // preserve keyword
                norm_idx += len as i32;
            } else {
                norm.push('v'); // normalize identifiers to 'v'
                norm_idx += 1;
            }
            continue;
        }

        // all other matches failed:
        norm.push(head.chars().next().unwrap());    // write first char of head to norm
        norm_idx += 1;      // progress next idx to be written to
        head = &head[1..];  // progress head
    }

    // final line always contains everything to end of norm
    line_ends.push(norm.chars().count() as i32);

    // return normalized text in struct for line number computations
    NormText {
        value: norm,
        line_ends: line_ends
    }

    /*
        mut head = program;
        norm = String::from("")
        norm_idx = 0
        line_ends = Vec::new() of i32

        while head not empty:

            if match_keyword_or_ident(head)
                head = rest (move the head past it)

                if ident:
                    norm.push('v')
                    norm_idx++

            if match_whitespace(head)
                head = rest

                norm.push(' ')  // space is generic ws
                norm_idx++

                account_for_newlines(matched, norm_idx, &mut line_ends)

            if match_type(head)
                head = rest // ignore
                account_for_newlines(matched, norm_idx, &mut line_ends)

            docstrings
                head = rest // ignore
                account_for_newlines(matched, norm_idx, &mut line_ends)

            comments
                head = rest // ignore
                account_for_newlines(matched, norm_idx, &mut line_ends)

            ELSE:
                write the first char of head to norm
                progress head by 1 char
                norm_idx++

        add norm.len() as last index to line_ends
        return NormText { value: norm, line_ends: line_ends }

    */

}

// extract a prefix of the hd that matches the given reg expression
// (ensure the given regex is prefixed with ^)
// - Some((match, rest, len)) gives the match, the rest of the string,
//      and length of the match
// - None indicates no match
fn extract_match<'a>(hd: &'a str, re: &Regex) -> Option<(&'a str, &'a str, usize)> {
    match re.find(hd) {
        Some(mat) => {
            let e = mat.end();
            // match, rest, and length of match
            Some((&hd[0..e], &hd[e..], e))
        },
        None => None,
    }
}

// match the longest identifier/keyword prefix of the given string,
// - Some((is_keyword, match, rest, len)) indicates if the match was a keyword or 
//      identifier, the match, the rest of input, and length of match
// - None indicates no keyword/identifier could be matched
fn match_keyword_or_ident(hd: &str) -> Option<(bool, &str, &str, usize)> {
    lazy_static! {
        // gleaned from pyret-lang/src/scripts/tokenize.js
        static ref KEYWORD: Regex = Regex::new(
            "^(raises-other-than|raises-satisfies|raises-violates|does-not-raise\
                |provide-types|otherwise:|load-table|is-roughly|descending|transform\
                |satisfies|is-not<=>|examples:|ascending|violates|type-let|sharing:\
                |sanitize|provide:|is-not=~|is-not==|examples|source:|reactor|provide\
                |newtype|include|extract|else if|because|where:|table:|shadow|select\
                |raises|module|method|letrec|is-not|import|hiding|extend|check:|block:\
                |with:|using|then:|sieve|order|is<=>|false|else:|check|cases|when|type\
                |true|row:|lazy|is=~|is==|from|else|doc:|data|var|spy|ref|rec|let|lam\
                |fun|for|end|ask|and|or|of|is|if|do|by|as)"
        ).unwrap();
        static ref IDENT: Regex = Regex::new(
            r"^([_a-zA-Z][_a-zA-Z0-9]*(?:-+[_a-zA-Z0-9]+)*)"
        ).unwrap();
    }

    let key_match = extract_match(hd, &KEYWORD);
    let id_match = extract_match(hd, &IDENT);

    match (key_match, id_match) {
        // both keyword & identifier match
        (Some(key), Some(id)) => {
            let (key_val, key_rest, key_len) = key;
            let (id_val, id_rest, id_len) = id;

            if id_len > key_len {
                // use identifier match
                Some((false, id_val, id_rest, id_len))
            } else {
                // use keyword match
                Some((true, key_val, key_rest, key_len))
            }
        },
        // only identifier
        (None, Some(id)) => Some((false, id.0, id.1, id.2)),

        // only keyword
        (Some(key), None) => Some((true, key.0, key.1, key.2)),

        // neither match
        (None, None) => None,
    }
}

// match a prefix of whitespace chars on the head
// - Some((matched, rest, len)) contains matched whitespace, rest of head, match length
// - None indicates no whitespace could be matched
fn match_whitespace(hd: &str) -> Option<(&str, &str, usize)> {
    unimplemented!();
}

// match a type annotation on the head (starting with ::)
// - Some((matched, rest, len)) contains matched type, rest of head, match length
// - None indicates no type could be matched
fn match_type(hd: &str) -> Option<(&str, &str, usize)> {
    // look for:
    // :: annot
    // -> annot
    // where annot is:
    // ( or ident
    unimplemented!();
}

// match a docstring on the head
// - Some((matched, rest, len)) contains matched docstring, rest of head, match length
// - None indicates no docstring match
fn match_docstring(hd: &str) -> Option<(&str, &str, usize)> {
    unimplemented!();
}

// match a comment on the head
// - Some((matched, rest, len)) contains matched comment, rest of head, match length
// - None indicates no comment matched
fn match_comment(hd: &str) -> Option<(&str, &str, usize)> {
    unimplemented!();
}

// read over a slice & add idx to le (line ends) for every newline encountered
fn account_for_newlines(slice: &str, idx: i32, le: &mut Vec<i32>) {
    unimplemented!();
}


#[cfg(test)]
mod normalize_tests {
    use super::*;
    use std::convert::TryInto;

    // generic testing form for normalize()
    // calls normalize() on input string & asserts output text value & line ends
    fn test_norm(input: &str, out_val: &str, out_line_ends: Vec<i32>) {
        let norm = normalize(/*String::from(input)*/ input);
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

}