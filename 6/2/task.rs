use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    //    let mut lookback = Vec::new();

    for s in lines {
        let t = s.unwrap();
        let l = t.as_bytes();
        let mut done = false;
        let mut c = 0;
        while !done {
            if all_different(&l[c..], 14) {
                done = true;
                println!("{:?}", c + 14);
            }
            c += 1;
        }
    }
}

fn all_different(s: &[u8], togo: usize) -> bool {
    if togo == 1 {
        return true;
    }

    let mut i = 1;
    let c = s[0];
    while i < togo {
        if c == s[i] {
            return false;
        }
        i += 1;
    }

    return all_different(&s[1..], togo - 1);
}
