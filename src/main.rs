extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let out = "Hello fellow Rustaceans!\n what is up";
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
