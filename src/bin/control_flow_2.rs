//! Learning basic control flow: match

use std::io;
use std::io::BufRead;

/// ### match is so powerful but we'll just introduce a bit here.
///
/// Complete the code piece to achieve the same logic as you have
/// done in `control_flow`.
///
/// 1. read into `buffer: String` and get a string.
/// 2. compare and print out messages. THIS TIME, WITH `match`!
///    WARNING: don't consider leading and tailing white characters
///    like "\n", "\t" and " ".
///    * When receiving "whoami", print "LCPU-RLG" with newline.
///    * When receiving "exit", print "EXIT" with newline.
///    * When receiving other input, just print it but WITHOUT leading and
///      tailing white characters.
///
/// Tips: you might want to use `trim()`, search for it yourself!
fn cf_match() {
    let mut buffer = String::new();
    let mut handle = io::stdin().lock();
    handle.read_line(&mut buffer).unwrap();
    match buffer.trim() {
        "whoami" => println!("LCPU_RLG"),
        "exit" => println!("EXIT"),
        _ => print!("{}", buffer.trim()),
    }
}

fn main() {
    cf_match();
}
