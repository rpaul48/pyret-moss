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
    // a char at index norm_idx in the normalized text corresponds to
    pub fn line_number(&self, norm_idx: i32) -> i32 {
        // iterate over all lines & the first norm char occurring *after* them
        for (zro_idx_line, first_char_after) in self.line_ends.iter().enumerate() {
            // if first char after this line is strictly after norm_idx,
            // then norm_idx takes place on this line (if it were on an earlier
            // line, it would've already terminated)
            if first_char_after > &norm_idx {
                return (zro_idx_line as i32) + 1;    // correct for 0-indexing
            }
        }

        // should not get here (last entry in line_ends is larger than any valid norm_idx)
        panic!("normalize: line_number called with invalid index {}", norm_idx);
    }
}

// replacement for all identifier names
// Note: unit tests may break if this is altered
// (written assuming 'v')
const NORM_IDENTIFIER: char = 'v';

// Remove/normalize any features from a program's text that
// shouldn't differentiate it from other programs:
//      1. normalize identifiers
//      2. remove whitespace
//      3. remove type annotations
//      4. remove docstrings
//      5. remove comments
// Returns the normalized string & enough info to map parts
// of the normalized text to line numbers in the original (LineMapping)
pub fn normalize(program: &str) -> NormText {
    let mut head: &str = &program;          // rest of program to be processed
    let mut norm = String::from("");        // normalized program text
    let mut norm_idx = 0;                   // next index to write to in norm text
    let mut line_ends = Vec::new();         // encodes line info (see NormText above)

    // while haven't seen entire program
    while !head.is_empty() {

        // ------- Whitespace -------
        if let Some((mat, rest, len)) = match_whitespace(head) {
            head = rest;    // jump over whitespace
            account_for_newlines(mat, norm_idx, &mut line_ends, false);
            continue;
        }

        // ------- Comments -------
        if let Some((mat, rest, len)) = match_comment(head) {
            head = rest;    // jump over comment
            account_for_newlines(mat, norm_idx, &mut line_ends, false);
            continue;
        }

        // ------- Docstrings -------
        if let Some((mat, rest, len)) = match_docstring(head) {
            head = rest;    // jump over docstring
            account_for_newlines(mat, norm_idx, &mut line_ends, false);
            continue;
        }

        // ------- Types -------
        if let Some((mat, rest, len)) = match_type(head) {
            head = rest;    // jump over annotation
            account_for_newlines(mat, norm_idx, &mut line_ends, false);
            continue;
        }

        // ------- String Literals -------
        if let Some((mat, rest, len)) = match_string_literal(head) {
            // account for newlines *before* incrementing norm_idx,
            // because whitespace is preserved in strings, and indices
            // for line ends within the literal need to be computed relative
            // to norm_idx before advancing *past* the string.
            account_for_newlines(mat, norm_idx, &mut line_ends, true);

            norm.push_str(mat); // write literal to norm
            norm_idx += len as i32;

            head = rest;    // jump over literal
            continue;
        }

        // ------- Keywords & Identifiers -------
        // (second to last test, because most general)
        if let Some((is_keyw, key_or_id)) = match_keyword_or_ident(head) {
            let (mat, rest, len) = key_or_id;

            head = rest;    // jump over keyword/ident

            if is_keyw {
                norm.push_str(mat); // preserve keyword
                norm_idx += len as i32;
            } else {
                norm.push(NORM_IDENTIFIER); // normalize identifiers
                norm_idx += 1;
            }
            continue;
        }

        // ------- Else -------
        norm.push(head.chars().next().unwrap());    // write first char of head to norm
        norm_idx += 1;      // progress next idx to be written to
        head = &head[1..];  // progress head
    }

    // final line always contains everything to end of norm
    line_ends.push(norm.chars().count() as i32);

    // return normalized text in struct for line number computations
    NormText { value: norm, line_ends: line_ends }
}

// A Match indicates a prefix of some string that represents some feature
// (i.e. whitespace, docstring, identifier, etc.)
// In (match, rest, len), match is the matching prefix, rest is the
// remaining slice of the original string, and len is the size of the match
type Match<'a> = (&'a str, &'a str, usize);

// extract the prefix of hd that matches the given reg expression, or None
// (ensure the given regex is prefixed with ^)
fn extract_match<'a>(hd: &'a str, re: &Regex) -> Option<Match<'a>> {
    match re.find(hd) {
        Some(mat) => {
            let e = mat.end();
            // match, rest, and length of match
            Some((&hd[..e], &hd[e..], e))
        },
        None => None,
    }
}

// extract longest prefix of whitespace if any, or None
fn match_whitespace(hd: &str) -> Option<Match> {
    lazy_static! {
        static ref WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
    }

    extract_match(hd, &WHITESPACE)
}

// extract longest comment prefix if any, or None
fn match_comment(hd: &str) -> Option<Match> {
    lazy_static! {
        // match single- and multi-line comments
        static ref COMMENT: Regex = Regex::new(
            r#"^((#\|(.|\n)*?\|#)|(#.*))"#
        ).unwrap();
    }

    extract_match(hd, &COMMENT)
}

// extract longest docstring prefix if any, or None
fn match_docstring(hd: &str) -> Option<Match> {
    lazy_static! {
        // match docstrings with '', "", and ``` quotes
        static ref DOC: Regex = Regex::new(
            r#"^doc:\s*((".*?")|('.*?')|(```(.|\n)*?```))"#
        ).unwrap();
    }

    extract_match(hd, &DOC)
}

// extract longest type annotation prefix if any, or None
fn match_type(hd: &str) -> Option<Match> {
    lazy_static! {
        // type parameters (i.e. <A, B, C>)
        static ref TYPE_PARAMS: Regex = Regex::new(
            r"^<([_a-zA-Z][-_a-zA-Z0-9]*(,\s*)?)*>"
        ).unwrap();
        // :: syntax preceding type annotations
        // (followed by mandatory whitespace)
        static ref ANNOT_PREFIX: Regex = Regex::new(
            r"^::\s+"
        ).unwrap();
        // -> syntax preceding output type annotations
        // (followed by arb. whitespace)
        static ref OUT_TYPE_PREFIX: Regex = Regex::new(
            r"^->\s*"
        ).unwrap();
        // simple (non-parenthesized) types
        static ref SIMPLE_TYPE: Regex = Regex::new(
            r"[_a-zA-Z][-_a-zA-Z0-9<>]*"
        ).unwrap();
    }

    // Given a match of an annotation prefix, attempt to parse the
    // following type, and combine them into a single match.
    // Also parse arbitrary whitespace following the type (so when types
    // are removed, no whitespace artifact will be left)
    fn parse_type<'a>(head: &'a str, prefix: Match) -> Option<Match<'a>> {
        let (pref_mat, pref_rest, pref_len) = prefix;

        if pref_rest.starts_with('(') {
            // try complex type (match everything within balanced parens)
            let mut open_parens = 1;    // count of open parentheses read so far
            let mut reader = pref_rest; // copy for reading through rest of type
            let mut typ_len = 1;        // number of matched chars in type

            while open_parens > 0 {
                reader = &reader[1..];  // move along
                typ_len += 1;

                // if out of input before balancing parens, fail
                if reader.is_empty() { return None; }

                // count opening/closing parens
                if reader.starts_with('(') {
                    open_parens += 1;
                } else if reader.starts_with(')') {
                    open_parens -= 1;
                }
            }

            // combine prefix & type matches
            let len = pref_len + typ_len;
            Some((&head[..len], &head[len..], len))

        } else {
            // try simple type
            match extract_match(pref_rest, &SIMPLE_TYPE) {
                Some((typ_mat, typ_rest, typ_len)) => {
                    // combine prefix & type matches
                    let len = pref_len + typ_len;
                    Some((&head[..len], &head[len..], len))
                },
                None => return None, // fail
            }
        }
    }

    // attempt to match :: prefix
    if let Some(colon_prefix) = extract_match(hd, &ANNOT_PREFIX) {
        return parse_type(hd, colon_prefix);
    }

    // attempt to match -> prefix
    if let Some(arrow_prefix) = extract_match(hd, &OUT_TYPE_PREFIX) {
        return parse_type(hd, arrow_prefix);
    }

    // finally, attempt to match type parameter list
    extract_match(hd, &TYPE_PARAMS)
}

// extract longest string literal prefix (single, double, or triple quoted) if any, or None
fn match_string_literal(hd: &str) -> Option<Match> {
    lazy_static! {
        // match all kinds of quoted strings
        static ref STRING_LIT: Regex = Regex::new(
            r#"^((".*?")|('.*?')|(```(.|\n)*?```))"#
        ).unwrap();
    }

    extract_match(hd, &STRING_LIT)
}

// extract longest identifier/keyword prefix if any, or None.
// boolean indicates true if a keyword was matched, false if identifier
fn match_keyword_or_ident(hd: &str) -> Option<(bool, Match)> {
    lazy_static! {
        // gleaned from pyret-lang/src/scripts/tokenize.js
        // ordered length descending so longest match is preferred
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
            // compare match lengths
            if id.2 > key.2 {
                // use identifier match
                Some((false, id))
            } else {
                // use keyword match
                Some((true, key))
            }
        },
        // only identifier
        (None, Some(id)) => Some((false, id)),

        // only keyword
        (Some(key), None) => Some((true, key)),

        // neither match
        (None, None) => None,
    }
}

// Read over a slice & add the index of the next normalized text char
// after each newline to the line ends (le) vector.
// If preserving newlines, next index will be index right after \n, otherwise
// idx parameter is used.
fn account_for_newlines(slice: &str, idx: i32, le: &mut Vec<i32>, preserve_newlines: bool) {
    for (i, c) in slice.chars().enumerate() {
        if c == '\n' {
            le.push(if preserve_newlines { idx + ((i + 1) as i32) } else { idx });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_number_info_preserved() {
        {
            let norm = normalize(
                "fun f(x):\n\
                    \tblock:\n\
                    \t\t5 * 10\n\
                    \t\tx + 10\n\
                    \t\t7 * x\n\
                    \tend\n\
                end");
            // norm: "funv(v):block:5*10v+107*vendend"
            assert_eq!(norm.line_number(0), 1);
            assert_eq!(norm.line_number(11), 2);
            assert_eq!(norm.line_number(14), 3);
            assert_eq!(norm.line_number(21), 4);
            assert_eq!(norm.line_number(23), 5);
            assert_eq!(norm.line_number(25), 6);
            assert_eq!(norm.line_number(30), 7);
        }
        {
            let norm = normalize(
                "x = ~100.051\n\
                y = ```long string literal over\n\
                        multiple lines```\n\
                lam(n): 5 * n end");
            // norm: "v=~100.051v=```long string literal over\nmultiple lines```lam(v):5*vend"
            assert_eq!(norm.line_number(9), 1);
            assert_eq!(norm.line_number(10), 2);
            assert_eq!(norm.line_number(39), 2);
            assert_eq!(norm.line_number(40), 3);
            assert_eq!(norm.line_number(57), 4);
            assert_eq!(norm.line_number(62), 4);
        }
    }

    // generic testing form for normalize()
    // calls normalize() on input string & asserts output text value & line ends
    fn test_norm(input: &str, out_val: &str, out_line_ends: Vec<i32>) {
        let norm = normalize(input);
        assert_eq!(norm.value, String::from(out_val));
        assert_eq!(norm.line_ends, out_line_ends);
    }

    #[test]
    fn empty_document() {
        test_norm(
            "",
            "",
            vec![0]);
    }

    #[test]
    fn whitespace_removed() {
        test_norm(
            "  \n \na = 1\n\t\t ",
            "v=1",
            vec![0, 0, 3, 3]);
        test_norm(
            "check:\n\n\t1 is \n2\nend",
            "check:1is2end",
            vec![6, 6, 9, 10, 13]);
    }

    #[test]
    fn identifiers_normalized() {
        test_norm(
            "name-1 = 7\nsecond_name = name-1 * name-1",
            "v=7v=v*v",
            vec![3, 8]);
    }

    #[test]
    fn types_removed() {
        test_norm(
            "x :: Number = 10\ny :: Boolean = true",
            "v=10v=true",
            vec![4, 10]);
        test_norm(
            "fun f(a :: Custom, b :: List)\n-> String:\n",
            "funv(v,v):",
            vec![9, 10, 10]);
        test_norm(
            "param :: List<List<InnerType>>",
            "v",
            vec![1]);
        test_norm(
            "complex :: ((Number -> String) -> \
            (List<String> -> List<List<Number>>)) = 10",

            "v=10",
            vec![4]);
        test_norm(
            "fun f<A, B, C>(x, y)",
            "funv(v,v)",
            vec![9]);
        test_norm(
            "fun do-it<Type,XYZ,_>(x :: Number) -> (String -> String):",
            "funv(v):",
            vec![8]);
        test_norm(
            "fun f \n <A, B>():",
            "funv():",
            vec![4, 7]);
        test_norm(
            "fun newlines-everywhere<X,Y,\n\nA, Z>(param\n::\nList<X>)\n\n ->\n Y\n:",
            "funv(v):",
            vec![4, 4, 6, 6, 7, 7, 7, 7, 8]);
        test_norm(
            "(id :: Number\n\n, id2)",
            "(v,v)",
            vec![2, 2, 5]);
    }

    #[test]
    fn docs_removed() {
        test_norm(
            "fun f():\n\
                \tdoc: \"docstring here\"\n\
                5\n\
            end",

            "funv():5end",
            vec![7, 7, 8, 11]);

        test_norm(
            "fun g():\n\
                doc: ```This is a longer docstring.\n\
                It takes place over multiple lines.```\n\
                0\n\
            end",

            "funv():0end",
            vec![7, 7, 7, 8, 11]);
    }

    #[test]
    fn comments_removed() {
        test_norm(
            "x = 1 # x is 1\ny = 2 # the value of y",
            "v=1v=2",
            vec![3, 6]);
        test_norm(
            "n = true #| commented code:\n\
            x = 15\n\
            y =\"string value\"\n\
            |#\n\
            m = false",

            "v=truev=false",
            vec![6, 6, 6, 6, 13]);
    }

    #[test]
    fn simple_func() {
        test_norm(
            "fun square(n): n * n end",
            "funv(v):v*vend",
            vec![14]);
    }

    #[test]
    fn preserves_string_literals() {
        test_norm(
            "my-literal = \"This is a string value\"",
            "v=\"This is a string value\"",
            vec![26]);
        test_norm(
            "x = 'single-quoted string; fun f(): end'",
            "v='single-quoted string; fun f(): end'",
            vec![38]);
        test_norm(
            "triple = ```here's a\ntriple-quote```",
            "v=```here's a\ntriple-quote```",
            vec![14, 29]);
    }

    #[test]
    fn keywords_and_otherwise_preserved() {
        // ensure any other syntactic elements are preserved
        test_norm(
            "import tables as T",
            "importvasv",
            vec![10]);
        test_norm(
            "if (5 * 2) < 10:\n\
                true\n\
            else:\n\
                false\n\
            end",

            "if(5*2)<10:trueelse:falseend",
            vec![11, 15, 20, 25, 28]);
        test_norm(
            "examples:\n\
                tmp = \"x = 5\"\n\
                tmp is tmp\n\
            end",

            "examples:v=\"x = 5\"visvend",
            vec![9, 18, 22, 25]);
    }

}
