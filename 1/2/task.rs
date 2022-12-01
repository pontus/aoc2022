use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut vec = Vec::new();
    let mut current = 0;
    for l in lines {
        let s = l.unwrap();

        if s != "" {
            current += s.parse::<i64>().unwrap();
        } else {
            vec.push(current);
            current = 0;
        }
    }

    vec.sort();

    let sum = vec.pop().unwrap()+vec.pop().unwrap()+vec.pop().unwrap();
    println!("{:?}", sum);
}
