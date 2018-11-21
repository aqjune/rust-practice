use std::io;
use std::fs::File;
extern crate compress;
use util;

pub fn run(input_path: String) {
    let f = File::open(input_path).expect("file not found");
    let mut buf:Vec<u8> = Vec::new();
    f.read_to_end(&mut buf);

    let mut compressed:Vec<u8> = Vec::new();
    compress::lz78::compress(buf, &mut compressed);
}
