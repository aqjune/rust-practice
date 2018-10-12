use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

mod naive;
mod kmp;
mod ahocorasick;

fn print_result(name:&str, queries:&Vec<Vec<char>>, res:&Vec<Vec<usize>>) {
    println!("<{}>", name);
    for idx in 0..queries.len() {
        let query_str:String = queries[idx].iter().collect();
        println!("Query: {}", query_str);
        for idx2 in 0..res[idx].len() {
            print!("{} ", res[idx][idx2]);
        }
        println!("");
    }
}

fn strip(chrs:&mut Vec<char>) {
    loop {
        match chrs.last().cloned() {
            None => break,
            Some (c) => {
                if !c.is_whitespace()
                { break; }
                chrs.pop();
            }
        }
    }
}

fn main() {
    let f = File::open(env::args().nth(1).expect("Missing argument (input file name)")).expect("file not found");
    let mut buff = BufReader::new(&f);
    let mut text = String::new();
    buff.read_line(&mut text);
    let mut text_vec:Vec<char> = text.chars().collect();
    strip(&mut text_vec);
    let mut queries_vec:Vec<Vec<char>> = Vec::new();

    loop {
        let mut query = String::new();
        match buff.read_line(&mut query) {
            Ok(sz) => {
                if sz == 0 {
                    break;
                }
                let mut query_vec:Vec<char> = query.chars().collect();
                strip(&mut query_vec);
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

    let mut ahocorasick_results:Vec<Vec<usize>> = Vec::new();
    ahocorasick::run(&text_vec, &queries_vec, &mut ahocorasick_results);
    print_result("aho-corasick", &queries_vec, &ahocorasick_results);
}
