use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut priorities: u32 = 0;
    let mut shared = Vec::new();

    for l in lines {
        let s = l.unwrap().trim().to_owned();
        let rucksack: Vec<u8> = s.into_bytes();
        let items = rucksack.len() / 2;
        let compartments = rucksack.split_at(items);

        let mut added = Vec::new();
        for p in compartments.0 {
            if compartments.1.contains(p) {
                if !added.contains(p) {
                    shared.push(p.to_owned());
                    added.push(p.to_owned());
                }
            }
        }
    }

    for p in shared {
        if p > 0x60 {
            priorities += (p as u32) - 0x60;
        } else {
            priorities += (p as u32) - 0x40 + 26;
        }
    }

    println!("{:?}", priorities);
}
