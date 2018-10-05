use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod naive;

fn main() {
    let f = File::open("test.input").expect("file not found");
    let mut buff = BufReader::new(&f);
    let mut text = String::new();
    buff.read_line(&mut text);
    let text_vec:Vec<char> = text.chars().collect();

    loop {
        let mut query = String::new();
        match buff.read_line(&mut query) {
            Ok(sz) => {
                if sz == 0 {
                    break;
                }
                println!("Query: {}", query);
                let mut query_vec:Vec<char> = query.chars().collect();
                loop {
                    match query_vec.last().cloned() {
                        None => break,
                        Some (c) => {
                            if !c.is_whitespace()
                            { break; }
                            query_vec.pop();
                        }
                    }
                }
                let mut result:Vec<usize> = Vec::new();

                naive::run(&text_vec, &query_vec, &mut result);

                for idx in result {
                    println!("{} ", idx)
                }
            },
            Err(_) => {}, // EOF
        }
    }
}
