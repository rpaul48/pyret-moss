/* error.rs: Graceful error handling */

use std::process;

// print an error message to stderr & exit, without panicking
pub fn err(message: &str) {
    eprintln!("Error: {}", message);
    process::exit(1);
}