#[macro_use] extern crate lazy_static;
extern crate regex;
use std::path::PathBuf;
use crate::fingerprint::Fingerprint;

mod fingerprint;
mod normalize;
mod file_io;
mod phase_i;
mod error;
mod cli;

// Sub represents a student submission.
// Depending on whether input submissions are directories or
// indiv. files, the dir_name field will be Some or None
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sub {
    pub dir_name: Option<PathBuf>,
    pub documents: Vec<Doc>
}

// Doc represents a file within a submission.
// Docs are initialized as Unprocessed (contents have not yet been
// read), and become Processed once they have been fingerprinted
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Doc {
    Unprocessed(PathBuf),
    Processed(PathBuf, Vec<Fingerprint>)
}

fn main() {
	// use std::env;
	// let args: Vec<String> = env::args().collect();
	// // println!("Arguments: {:?}", args);
	// let (sub_dir, opts) = cli::parse_args(&args);
	// println!("Submission directory: {}", sub_dir.display());
	// println!("Optional args: {:#?}", opts);


	use phase_i::{make_ignore_set, analyze_subs};
	use std::path::Path;
	use crate::fingerprint::{self, Fingerprint};
	use crate::normalize::normalize;




	// let a = "provide *\n\
	// \n\
	// data Structure<T>:\n\
	// 	\t| variant(field :: T)\n\
	// end";

	// let norm = normalize::normalize(&a[..]);
	// let fps = fingerprint::fingerprint(norm);

	// println!("{:?}", fps);
	// println!("{} fingerprints.", fps.len());



	let k: i32 = 15;
	let t: i32 = 17;

	let mut sub1 = Sub {
		dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub1")),
		documents: vec![
			Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub1/common.arr")),
			Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub1/main.arr"))
		]
	};
	let mut sub2 = Sub {
		dir_name: Some(PathBuf::from("test-dirs/test/multi-file/sub2")),
		documents: vec![
			Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub2/common.arr")),
			Doc::Unprocessed(PathBuf::from("test-dirs/test/multi-file/sub2/main.arr"))
		]
	};

	let mut submissions = vec![&mut sub1, &mut sub2];

	if let Ok(set) = make_ignore_set(&Path::new("test-dirs/test/ignore"), k, t) {
		println!("Ignoring {:?}", set);
		println!("Analysis output: {:#?}", analyze_subs(&mut submissions, Some(set), k, t));
	}
}
