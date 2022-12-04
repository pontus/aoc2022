use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut counter = 0;

    for l in lines {
        let line = l.unwrap().to_owned();
        let mut s = line.trim().split(",");
        let mut left = s.next().to_owned().to_owned().unwrap().split("-");
        let mut right = s.next().to_owned().unwrap().split("-");

        let llow = left.next().unwrap().parse::<u32>().unwrap();
        let lhigh = left.next().unwrap().parse::<u32>().unwrap();

        let rlow = right.next().unwrap().parse::<u32>().unwrap(      );
        let rhigh = right.next().unwrap().parse::<u32>().unwrap();

        if (rlow <= llow && rhigh >= lhigh) || (rlow >= llow && rhigh <= lhigh) {
            counter += 1;
        }
    }

    println!("{:?}", counter);
}
