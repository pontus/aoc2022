use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut cycles = 0;
    let mut x = 1 as i32;
    let mut signal = Vec::new();

    for l in lines {
        let line = l.unwrap();
        let mut s = line.split(" ");

        let d = s.next().unwrap();

        signal.push(x);
        if d == "noop" {
            cycles += 1;
        } else {
            signal.push(x);

            cycles += 2;
            let add = s.next().unwrap().parse::<i32>().unwrap();
            x += add;
        }
    }

    print!(
        "Response: {}\n",
        signal[19] * 20
            + signal[59] * 60
            + signal[99] * 100
            + signal[139] * 140
            + signal[179] * 180
            + signal[219] * 220
    );
}
