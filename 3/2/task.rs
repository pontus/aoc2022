use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut priorities: u32 = 0;
    let mut shared: Vec<Vec<u8>> = Vec::new();
    let mut badges = Vec::new();
    let mut gcount = 0;

    for l in lines {
        if gcount == 3 {
            for p in shared[0].to_owned() {
                if shared[1].contains(&p) && shared[2].contains(&p) {
                    badges.push(p.to_owned());
                    break;
                }
            }
            gcount = 1;
            shared = Vec::new();
        } else {
            gcount += 1;
        }

        let s = l.unwrap().trim().to_owned();
        let rucksack: Vec<u8> = s.into_bytes();

        shared.push(rucksack.to_owned());
    }
    for p in shared[0].to_owned() {
        if shared[1].contains(&p) && shared[2].contains(&p) {
            badges.push(p.to_owned());
            break;
        }
    }

    for p in badges {
        if p > 0x60 {
            priorities += (p as u32) - 0x60;
        } else {
            priorities += (p as u32) - 0x40 + 26;
        }
    }

    println!("{:?}", priorities);
}
