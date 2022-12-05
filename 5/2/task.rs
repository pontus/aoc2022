use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut read_stacks = true;
    let mut stacks = Vec::new();

    for _p in 0..12 {
        stacks.push(Vec::<char>::new());
    }

    for l in lines {
        let line = l.unwrap().to_owned();
        if line == "" {
            read_stacks = false;

            for p in 0..11 {
                stacks[p].reverse();
            }
            continue;
        }

        if read_stacks {
            let s = line.into_bytes();
            if s[1] != 0x31 {
                let mut i = 1;
                while i < 35 {
                    if s.len() > i && s[i] != 0x20 {
                        let c = s[i] as char;

                        print!("Stack {:?}, pushing {:?} \n", (i / 4), c);

                        stacks[i / 4].push(c);
                    }
                    i += 4;
                }
            }
        } else {
            let mut s = line.trim().split(" ");
            s.next();
            let num = s.next();
            s.next();
            let src = s.next();
            s.next();
            let dst = s.next();

            let n = num.unwrap().parse::<u16>().unwrap();
            let s = src.unwrap().parse::<usize>().unwrap() - 1;
            let d = dst.unwrap().parse::<usize>().unwrap() - 1;

            print!("Moving {:?} crates from {:?} to {:?}\n", n, s, d);

            let mut i = 0;
            let mut tmp = Vec::new();
            while i < n {
                tmp.push(stacks[s].pop().unwrap());
                print!("Moving {:?} \n", tmp);
                i += 1;
            }

            while tmp.len() > 0 {
                stacks[d].push(tmp.pop().unwrap());
            }
        }
    }
    print!("\n\n");
    for mut p in stacks {
        if p.len() > 0 {
            let q = p.pop().unwrap();
            print!("{:?} ", q);
        }
    }

    print!("\n\n ");
}
