use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    //    let mut lookback = Vec::new();

    for s in lines {
        let mut t = s.unwrap();
        let mut l = t.as_bytes();
        let mut done = false;
        let mut c = 0;
        while !done {
            if (l[c] != l[c + 1])
                && (l[c + 1] != l[c + 2])
                && (l[c + 2] != l[c + 3])
                && (l[c + 3] != l[c + 1])
                && (l[c] != l[c + 2])
                && (l[c] != l[c + 3])
            {
                done = true;
                println!("{:?}", c + 4);
            }
            c += 1;
        }
    }
}
