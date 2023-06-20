extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::str;

fn main() {
    let width = 24;
    // let out = "Hello fellow Rustaceans!";
    // println!("{:?}", out.as_bytes());

    let byte_out = vec![
        72, 101, 108, 108, 111, 32, 102, 101, 108, 108, 111, 119, 32, 82, 117, 115, 116, 97, 99,
        101, 97, 110, 115, 33,
    ];

    let mut writer = BufWriter::new(stdout());
    say(str::from_utf8(&byte_out).unwrap(), width, &mut writer).unwrap();
}
