use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut cycles = 0;
    let mut x = 1 as i32;
    let mut signal = Vec::new();

    for l in lines {
        let pos = cycles % 40;
        if pos == 0 {
            print!("\n");
        }
        if x - 1 <= pos && pos <= x + 1 {
            print!("#");
        } else {
            print!(".");
        }

        let line = l.unwrap();
        let mut s = line.split(" ");

        let d = s.next().unwrap();

        signal.push(x);
        if d != "noop" {
            signal.push(x);

            cycles += 1;
            if cycles % 40 == 0 {
                print!("\n");
            }
            if x - 1 <= (cycles % 40) && (cycles % 40) <= x + 1 {
                print!("#");
            } else {
                print!(".");
            }

            let add = s.next().unwrap().parse::<i32>().unwrap();
            x += add;
        }
        cycles += 1;
        if cycles % 40 == 0 {
            print!("\n");
        }
    }
}
