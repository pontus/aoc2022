use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut most = 0;

    let mut current = 0;
    for l in lines {
        let s = l.unwrap();

        if s != "" {
            current += s.parse::<i64>().unwrap();
        } else {
            if most < current {
                most = current;
            }
            current = 0;
        }
    }

    println!("{:?}", most);
}
