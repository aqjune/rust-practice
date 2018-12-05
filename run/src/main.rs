#[macro_use]
extern crate scan_rules;

use std::env;

mod run_stringmatch;
mod run_stringmatch2d;
mod run_compress;
mod run_voronoi;
mod util;

fn main() {
    let helpermsg0: String = [
        "run stringmatch   <input(ex:inputs/stringmatch/test1.input)>",
        "run stringmatch2d <input(ex:inputs/stringmatch2d/test1.input)>",
        "run compress      <input(ex:inputs/compress/infile.txt)>",
        "run voronoi       <input(ex:inputs/voronoi/test1.input)>"
    ].join("\n");
    let helpermsg = helpermsg0.as_str();
    let opt = env::args().nth(1).expect(helpermsg);
    if opt == "stringmatch" {
        let input_path = env::args().nth(2).expect(helpermsg);
        run_stringmatch::run(input_path);
    } else if opt == "stringmatch2d" {
        let input_path = env::args().nth(2).expect(helpermsg);
        run_stringmatch2d::run(input_path);
    } else if opt == "compress" {
        let input_path = env::args().nth(2).expect(helpermsg);
        run_compress::run(input_path);
    } else if opt == "voronoi" {
        let input_path = env::args().nth(2).expect(helpermsg);
        run_voronoi::run(input_path);
    } else {
        println!("{}", helpermsg);
    }
}
