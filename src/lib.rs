use std::fs;
use std::sync::Once;

static INIT: Once = Once::new();

/// Run some initialization code to make sure everything will work as expected.
pub fn init() {
    // We can only initialize the logger once before it panics, so ensure this
    // is only called once.
    INIT.call_once(|| {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "info");
        }

        pretty_env_logger::init();
    });
}

/// Load the input from ./inputs/{name}.txt as a string.
pub fn load_input(name: &str) -> String {
    // Read entirety of file into memory
    let mut input = fs::read_to_string(format!("./inputs/{}.txt", name)).expect("can't open file");

    // Trim the file to ensure any trailing whitespace is removed
    let len = input.trim().len();
    input.truncate(len);

    input
}
