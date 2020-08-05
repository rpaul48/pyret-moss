#[macro_use] extern crate lazy_static;
extern crate regex;
use std::path::Path;
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
#[derive(Debug)]
pub struct Sub<'a> {
    pub dir_name: Option<&'a Path>,
    pub documents: Vec<Doc<'a>>
}

// Doc represents a file within a submission.
// Docs are initialized as Unprocessed (contents have not yet been
// read), and become Processed once they have been fingerprinted
#[derive(Debug)]
pub enum Doc<'a> {
    Unprocessed(&'a Path),
    Processed(&'a Path, Vec<Fingerprint>)
}

fn main() {
	use std::env;
	let args: Vec<String> = env::args().collect();
	println!("Arguments: {:?}", args);







	

	// use phase_i::{make_ignore_set, analyze_subs};
	// use std::path::Path;
	// use crate::fingerprint::{self, Fingerprint};
	// use crate::normalize::normalize;

	// let a = "provide *\n\
	// \n\
	// data Structure<T>:\n\
	// 	\t| variant(field :: T)\n\
	// end";

	// let norm = normalize::normalize(&a[..]);
	// let fps = fingerprint::fingerprint(norm);

	// println!("{:?}", fps);
	// println!("{} fingerprints.", fps.len());


	// let mut sub1 = Sub {
	// 	dir_name: Some(&Path::new("./test-dirs/tinker/multi-dir/sub1")),
	// 	documents: vec![
	// 		Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub1/common.arr")),
	// 		Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub1/main.arr"))
	// 	]
	// };
	// let mut sub2 = Sub {
	// 	dir_name: Some(&Path::new("./test-dirs/tinker/multi-dir/sub2")),
	// 	documents: vec![
	// 		Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub2/common.arr")),
	// 		Doc::Unprocessed(&Path::new("./test-dirs/tinker/multi-dir/sub2/main.arr"))
	// 	]
	// };

	// let mut submissions = vec![&mut sub1, &mut sub2];

	// if let Ok(set) = make_ignore_set(&Path::new("./test-dirs/tinker/ignore")) {
	// 	println!("Ignoring {:?}", set);
	// 	println!("Analysis output: {:#?}", analyze_subs(&mut submissions, Some(set)));
	// }
}