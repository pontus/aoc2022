use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};

extern crate matrix;

use matrix::prelude::*;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let dim = 99;

    let mut m = Compressed::zero((dim, dim));
    let mut i = 0;

    for l in lines {
        let line = l.unwrap();
        let s = line.as_bytes();
        let mut j = 0;
        while j < dim {
            let n = s[j] - 0x30;
            m.set((j, i), n);
            //print!("{} {} = {}\n", j, i, n);
            j += 1;
        }
        i += 1;
    }

    let mut highest_score = 0;
    let mut x = 0;
    while x < m.columns {
        let mut y = 0;

        while y < m.rows {
            let score = scenic_score(x, y, &m);

            print!("Score for {},{} is {}\n", x, y, score);

            highest_score = max(score, highest_score);
            y += 1;
        }
        x += 1;
    }

    print!("Highest score: {}\n", highest_score);
}

fn scenic_score(x: usize, y: usize, m: &Compressed<u8>) -> u32 {
    let h = m.get((x, y));

    if x == 0 || y == 0 || x == m.columns || y == m.rows {
        return 0;
    }

    // Left

    let mut ltrees = 0;
    let mut i = x - 1;
    while i >= 0 {
        ltrees += 1;
        if m.get((i, y)) >= h || i == 0 {
            break;
        }

        i -= 1;
    }

    // Right
    let mut rtrees = 0;
    let mut i = x + 1;
    while i < m.columns {
        rtrees += 1;
        if m.get((i, y)) >= h {
            break;
        }
        i += 1;
    }

    // Up

    let mut utrees = 0;
    let mut i = y - 1;
    while i >= 0 {
        utrees += 1;

        if m.get((x, i)) >= h || i == 0 {
            break;
        }

        i -= 1;
    }

    // Down
    let mut dtrees = 0;
    let mut i = y + 1;
    while i < m.rows {
        dtrees += 1;
        if m.get((x, i)) >= h {
            break;
        }
        i += 1;
    }

    print!(
        "Count for {},{} is {} {} {} {}\n",
        x, y, ltrees, rtrees, utrees, dtrees
    );

    return ltrees * rtrees * utrees * dtrees;
}
