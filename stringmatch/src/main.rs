use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod naive;
mod kmp;

fn print_result(name:&str, res:&Vec<usize>) {
    print!("{}: ", name);
    for idx in res { print!("{} ", idx) }
    println!("")
}

fn main() {
    let f = File::open("test2.input").expect("file not found");
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
                let query2:String = query_vec.iter().collect();
                println!("Query: {}", query2);
                let mut result:Vec<usize> = Vec::new();

                naive::run(&text_vec, &query_vec, &mut result);
                print_result("naive", &result);
                result.clear();
                kmp::run(&text_vec, &query_vec, &mut result);
                print_result("kmp", &result);
            },
            Err(_) => {}, // EOF
        }
    }
}
