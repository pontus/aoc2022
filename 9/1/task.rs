use std::fs::File;
use std::io::{self, BufRead};

extern crate matrix;

use matrix::prelude::*;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let dim = 1600;
    let mut m = Compressed::zero((dim, dim));

    let mut hx = dim / 2;
    let mut hy = dim / 2;
    let mut tx = dim / 2;
    let mut ty = dim / 2;

    m.set((tx, ty), 1 as u8);

    for l in lines {
        let line = l.unwrap();
        let mut s = line.split(" ");

        let d = s.next().unwrap();
        let mut todo = s.next().unwrap().parse::<u16>().unwrap();
        print!("Stepping {} {}\n", d, todo);

        while todo > 0 {
            // print!("Stepping {}\n", d);
            match d {
                "U" => hy -= 1,
                "D" => hy += 1,
                "R" => hx += 1,
                "L" => hx -= 1,
                _ => print!("Something blew up"),
            }

            if !is_adjacent(hx, hy, tx, ty) {
                if hx == tx {
                    if hy > ty {
                        ty += 1;
                    } else if hy < ty {
                        ty -= 1;
                    }
                } else if hy == ty {
                    if hx > tx {
                        tx += 1;
                    } else if hx < tx {
                        tx -= 1;
                    }
                } else {
                    if hx > tx {
                        if hy > ty {
                            tx += 1;
                            ty += 1;
                        } else {
                            tx += 1;
                            ty -= 1;
                        }
                    } else if hx < tx {
                        if hy > ty {
                            tx -= 1;
                            ty += 1;
                        } else {
                            tx -= 1;
                            ty -= 1;
                        }
                    }
                }
            }

            print!("New coordinates {} {}\n", hx, hy);

            m.set((tx, ty), 1 as u8);

            todo -= 1;
        }
    }

    let mut visited = 0;
    let mut x = 0;
    while x < m.columns {
        let mut y = 0;

        while y < m.rows {
            // print!("Checking  {:?}, {:?} \n", x, y);
            if m.get((x, y)) > 0 {
                visited += 1;
            }
            y += 1;
        }
        x += 1;
    }

    print!("Visited: {}\n", visited);
}

fn is_adjacent(hx: usize, hy: usize, tx: usize, ty: usize) -> bool {
    if hx == tx && hy == ty {
        return true;
    }

    if hx > 0 && tx == (hx - 1) && (hy > 0 && ty == hy - 1 || ty == hy || ty == (hy + 1)) {
        return true;
    }

    if tx == hx && (hy > 0 && ty == hy - 1 || ty == hy || ty == (hy + 1)) {
        return true;
    }

    if tx == (hx + 1) && (hy > 0 && ty == hy - 1 || ty == hy || ty == (hy + 1)) {
        return true;
    }

    return false;
}
