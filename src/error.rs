/* error.rs: Graceful error handling */

// print an error message to stderr & exit, without panicking
#[macro_export]
macro_rules! err {
    // no args
    () => {
        eprintln!("An error occurred.");
        std::process::exit(1);
    };
    // print error message
    ($mes:expr) => {
        eprintln!("Error: {}", $mes);
        std::process::exit(1);
    };
    // print formatted error message
    ($fmt:expr, $($args:tt)*) => {
        eprintln!("Error: {}", std::format_args!($fmt, $($args)*));
        std::process::exit(1);
    };
}