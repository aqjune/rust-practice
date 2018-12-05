use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
extern crate stringmatch;
use util;

fn print_and_check_result(
    name: &str,
    queries: &Vec<Vec<char>>,
    res: &Vec<Vec<usize>>,
    answer: Option<&Vec<Vec<usize>>>,
) {
    println!("<{}>", name);
    for idx in 0..res.len() {
        let query_str: String = queries[idx].iter().collect();
        println!("Query: {}", query_str);
        for idx2 in 0..res[idx].len() {
            print!("{} ", res[idx][idx2]);
        }
        println!("");
    }
    match answer {
        None => return,
        Some(answ) => if answ != res {
            println!("Result does not match with the output from naive algorithm!!");
            ::std::process::exit(1);
        },
    }
}

pub fn run(input_path: String) {
    let f = File::open(input_path).expect("file not found");
    let mut buff = BufReader::new(&f);
    let mut text = String::new();
    buff.read_line(&mut text).expect("empty input");
    let mut text_vec: Vec<char> = text.chars().collect();
    util::strip(&mut text_vec);
    let mut queries_vec: Vec<Vec<char>> = Vec::new();

    loop {
        let mut query = String::new();
        match buff.read_line(&mut query) {
            Ok(sz) => {
                if sz == 0 {
                    break;
                }
                let mut query_vec: Vec<char> = query.chars().collect();
                util::strip(&mut query_vec);
                queries_vec.push(query_vec);
            }
            Err(_) => {} // EOF
        }
    }

    let mut naive_results: Vec<Vec<usize>> = Vec::new();
    stringmatch::naive::run(&text_vec, &queries_vec, &mut naive_results);
    print_and_check_result("naive", &queries_vec, &naive_results, None);

    let mut kmp_results: Vec<Vec<usize>> = Vec::new();
    let text_u16: Vec<u16> = text_vec.iter().map(util::char_to_u16).collect::<Vec<_>>();
    for i in 0..queries_vec.len() {
        let query_u16: Vec<u16> = queries_vec[i]
            .iter()
            .map(util::char_to_u16)
            .collect::<Vec<_>>();
        let mut result: Vec<usize> = Vec::new();
        let mut pfxsfx: Vec<usize> = vec![0; query_u16.len()];
        stringmatch::kmp::build_pfxsfx(&query_u16, &mut pfxsfx);
        stringmatch::kmp::run(&text_u16, &query_u16, &pfxsfx, &mut result);
        kmp_results.push(result);
    }
    print_and_check_result("kmp", &queries_vec, &kmp_results, Some(&naive_results));

    let mut ahocorasick_results: Vec<Vec<usize>> = Vec::new();
    let mut ahc_t: stringmatch::ahocorasick::AhcTrie = stringmatch::ahocorasick::AhcTrie {
        trie: Vec::new(),
        failure: Vec::new(),
        output: Vec::new(),
    };
    stringmatch::ahocorasick::build_trie(&queries_vec, &mut ahc_t);
    //stringmatch::ahocorasick::print_trie(&ahc_t);
    stringmatch::ahocorasick::run(&text_vec, &queries_vec, &ahc_t, &mut ahocorasick_results);
    print_and_check_result(
        "aho-corasick",
        &queries_vec,
        &ahocorasick_results,
        Some(&naive_results),
    );
}
