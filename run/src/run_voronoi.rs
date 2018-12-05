use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
extern crate voronoi;

pub fn run(input_path: String) {
    let f = File::open(input_path).expect("file not found");
    let br = BufReader::new(f);
    let mut input:Vec<(f64, f64)> = Vec::new();
    for res_strline in br.lines() {
        let strline = res_strline.unwrap();
        let mut words = strline.trim().split_whitespace();
        let x:f64 = words.next().unwrap().parse().unwrap();
        let y:f64 = words.next().unwrap().parse().unwrap();
        input.push((x, y))
    }
    let mut output:Vec<voronoi::line::Line> = Vec::new();
    
    voronoi::bruteforce::run(&input, &mut output);

    for line in output {
        println!("{} {} {} {} {}", line.xbeg, line.ybeg, line.xend, line.yend, line.finite);
    }
}
