use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod naive;
mod kmp;

fn print_result(name:&str, queries:&Vec<Vec<char>>, res:&Vec<Vec<usize>>) {
    print!("{}: ", name);
    for idx in 0..queries.len() {
        let query_str:String = queries[idx].iter().collect();
        println!("Query: {}", query_str);
        for idx2 in 0..res[idx].len() {
            print!("{} ", res[idx][idx2]);
        }
        println!("");
    }
}

fn main() {
    let f = File::open("test2.input").expect("file not found");
    let mut buff = BufReader::new(&f);
    let mut text = String::new();
    buff.read_line(&mut text);
    let text_vec:Vec<char> = text.chars().collect();
    let mut queries_vec:Vec<Vec<char>> = Vec::new();

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
                queries_vec.push(query_vec);
            },
            Err(_) => {}, // EOF
        }
    }

    let mut naive_results:Vec<Vec<usize>> = Vec::new();
    naive::run(&text_vec, &queries_vec, &mut naive_results);
    print_result("naive", &queries_vec, &naive_results);

    let mut kmp_results:Vec<Vec<usize>> = Vec::new();
    kmp::run(&text_vec, &queries_vec, &mut kmp_results);
    print_result("kmp", &queries_vec, &kmp_results);
}
