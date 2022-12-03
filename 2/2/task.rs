use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut score = 0;

    for l in lines {
        let s = l.unwrap();
        let guide: Vec<&str> = s.trim().split(" ").collect();

        score += match guide[1] {
            "X" => {
                0 + match guide[0] {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    &_ => todo!(),
                }
            }
            "Y" => {
                3 + match guide[0] {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    &_ => todo!(),
                }
            }
            "Z" => {
                6 + match guide[0] {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    &_ => todo!(),
                }
            }
            &_ => todo!(),
        }
    }

    println!("{:?}", score);
}
