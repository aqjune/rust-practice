use std::io::Read;
use std::fs::File;
extern crate compress;

pub fn run(input_path: String) {
    let mut f = File::open(input_path).expect("file not found");
    let mut buf:Vec<u8> = Vec::new();
    let mut buf2:Vec<u8> = Vec::new();
    f.read_to_end(&mut buf).expect("empty result");

    let mut compressed:Vec<u8> = Vec::new();
    compress::lz78::compress(&buf, &mut compressed, compress::lz78::Lz78EncodeProc::UseCharHighBit);
    compress::lz78::decompress(&compressed, &mut buf2, compress::lz78::Lz78EncodeProc::UseCharHighBit);
    
    if buf != buf2 {
        println!("Differ!!");
    } else {
        let orgsz = buf.len();
        let compsz = compressed.len();
        let rate = (1.0 - (compsz as f64) / (orgsz as f64)) * 100.0;
        println!("Okay, original size vs. compressed size: {} vs {} ({}% compressed)", orgsz, compsz, rate);
    }
}
