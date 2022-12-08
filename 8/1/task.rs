use std::fs::File;
use std::io::{self, BufRead};

extern crate matrix;

use matrix::prelude::*;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut m = Compressed::zero((99, 99));
    let mut i = 0;

    for l in lines {
        let line = l.unwrap();
        let s = line.as_bytes();
        let mut j = 0;
        while j < 99 {
            let n = s[j] - 0x30;
            m.set((j, i), n);
            //print!("{} {} = {}\n", j, i, n);
            j += 1;
        }
        i += 1;
    }

    let mut visible = 0;
    let mut notvisible = 0;
    let mut x = 0;
    while x < m.columns {
        let mut y = 0;

        while y < m.rows {
            print!("Checking if {},{} is visible\n", x, y);

            if is_visible(x, y, &m) {
                print!("yes\n");
                visible += 1;
            } else {
                notvisible += 1;
            }
            y += 1;
        }
        x += 1;
    }

    print!("Visible: {}\n", visible);
    print!("Not visible: {}\n", notvisible);
}

fn is_visible(x: usize, y: usize, m: &Compressed<u8>) -> bool {
    let h = m.get((x, y));

    if x == 0 || y == 0 || x == m.columns || y == m.rows {
        return true;
    }

    // Left

    let mut i = x - 1;
    while i >= 0 {
        if m.get((i, y)) >= h {
            break;
        }

        if i == 0 {
            return true;
        }
        i -= 1;
    }

    // Right
    let mut i = x + 1;
    while i < m.columns {
        if m.get((i, y)) >= h {
            break;
        }
        i += 1;
    }
    if i == m.columns {
        return true;
    }

    // Up

    let mut i = y - 1;
    while i >= 0 {
        if m.get((x, i)) >= h {
            break;
        }

        if i == 0 {
            return true;
        }

        i -= 1;
    }

    // Down
    let mut i = y + 1;
    while i < m.rows {
        if m.get((x, i)) >= h {
            break;
        }
        i += 1;
    }
    if i == m.rows {
        return true;
    }

    return false;
}
