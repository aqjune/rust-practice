use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("test.input").expect("file not found");
    let mut buff = BufReader::new(&f);
    let mut text = String::new();
    let mut query = String::new();
    buff.read_line(&mut text);
    buff.read_line(&mut query);

    let text_vec:Vec<char> = text.chars().collect();
    let query_vec:Vec<char> = query.chars().collect();

    for idx1 in 0..(text_vec.len() - query_vec.len()) {
        let mut matched = true;
        for idx2 in 0..(query_vec.len() - 1) { // ignore \n
            if (text_vec[idx1 + idx2] != query_vec[idx2]) {
                matched = false;
                break;
            }
        }
        if (matched) {
            println!("{}", idx1);
        }
    }
}
