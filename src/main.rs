#[macro_use] extern crate lazy_static;
extern crate regex;

mod fingerprint;
mod normalize;
mod file_io;
mod phase_i;
mod error;

/*
User-available parameters:
	- ignore boilerplate code (indicate a dir)
	- single-dir mode: submissions are assumed to be each 1 doc
	- k: noise threshold
	- t: guarantee threshold
	- result location: where the program's result summary will be written (default stdout)
	- limit max number of pairs of subs to report on in output
*/

use phase_i::{make_ignore_set, analyze_subs};
use std::path::Path;
use crate::fingerprint::Fingerprint;
use crate::file_io::{Sub, Doc};

fn main() {
	let mut sub1 = Sub {
		dir_name: Some(&Path::new("./test-dirs/tinker/multi-dir/sub1")),
		documents: vec![
			Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub1/common.arr")),
			Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub1/main.arr"))
		]
	};
	let mut sub2 = Sub {
		dir_name: Some(&Path::new("./test-dirs/tinker/multi-dir/sub2")),
		documents: vec![
			Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub2/common.arr")),
			Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub2/main.arr"))
		]
	};

	let mut submissions = vec![&mut sub1, &mut sub2];

	if let Ok(set) = make_ignore_set(&Path::new("./test-dirs/tinker/ignore")) {
		println!("Ignoring {:?}", set);
		println!("Analysis output: {:#?}", analyze_subs(&mut submissions, Some(set)));
	}
}