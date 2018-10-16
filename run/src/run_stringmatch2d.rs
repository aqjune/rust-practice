use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
extern crate stringmatch2d;
use util;


fn print_and_check_result(name:&str, query:&Vec<Vec<char>>,
                          res:&Vec<(usize, usize)>,
                          answer:Option<&Vec<(usize, usize)>>) {
    println!("<{}>", name);
    println!("Query:");
    for idx in 0..query.len() {
        let query_str:String = query[idx].iter().collect();
        println!("  {}", query_str);
    }
    for idx2 in 0..res.len() {
      print!("(y={}, x={}) ", res[idx2].0, res[idx2].1);
    }
    println!("");
    match answer {
      None => return,
      Some(answ) =>
        if answ != res {
          println!("Result does not match with the output from naive algorithm!!");
          ::std::process::exit(1);
        }
    }
}

fn read_text2d(buff:&mut BufReader<&File>, text_vec:&mut Vec<Vec<char>>, n:usize) {
  for _ in 0..n {
    let mut text = String::new();
    match buff.read_line(&mut text) {
      Ok(sz) => {
        if sz == 0 { break; }
        let mut line_vec:Vec<char> = text.chars().collect();
        util::strip(&mut line_vec);
        if line_vec.len() == 0 {
          // empty line! end of text
          break;
        } else {
          text_vec.push(line_vec);
        }
      },
      Err(_) => {}, // EOF
    }
  }
}

pub fn run(input_path:String) {
  let f = File::open(input_path).expect("file not found");
  let mut buff = BufReader::new(&f);
  let mut a_str = String::new();
  match buff.read_line(&mut a_str) {
    Ok(_) => {},
    Err(_) => {
      println!("Illegal format");
      ::std::process::exit(1);
    },
  };
  let_scan!(&a_str; (let n:usize, let m: usize));
  println!("{} {}", n, m);
  let mut query_vec:Vec<Vec<char>> = Vec::new();
  let mut text_vec:Vec<Vec<char>> = Vec::new();
  read_text2d(&mut buff, &mut query_vec, n);
  read_text2d(&mut buff, &mut text_vec, m);

  let mut naive_results:Vec<(usize, usize)> = Vec::new();
  stringmatch2d::naive::run(&text_vec, &query_vec, &mut naive_results);
  print_and_check_result("naive", &query_vec, &naive_results, None);

  let mut bakerbird_results:Vec<(usize, usize)> = Vec::new();
  stringmatch2d::bakerbird::run(&text_vec, &query_vec, &mut bakerbird_results);
  print_and_check_result("baker-bird", &query_vec, &bakerbird_results, Some(&naive_results));
}