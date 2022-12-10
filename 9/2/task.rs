use std::fs::File;
use std::io::{self, BufRead};

extern crate matrix;

use matrix::prelude::*;

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let dim = 1600;
    let mut m = Compressed::zero((dim, dim));
    let mut poses = Vec::new();

    for _p in 0..10 {
        poses.push((dim / 2, dim / 2));
    }

    m.set((dim / 2, dim / 2), 1 as u8);

    for l in lines {
        let line = l.unwrap();
        let mut s = line.split(" ");

        let d = s.next().unwrap();
        let mut todo = s.next().unwrap().parse::<u16>().unwrap();
        print!("Stepping {} {}\n", d, todo);

        while todo > 0 {
            // print!("Stepping {}\n", d);
            match d {
                "U" => poses[0].1 -= 1,
                "D" => poses[0].1 += 1,
                "R" => poses[0].0 += 1,
                "L" => poses[0].0 -= 1,
                _ => print!("Something blew up"),
            }

            for i in 1..10 {
                if !is_adjacent(poses[i - 1], poses[i]) {
                    print!(
                        "{},{} ({}) and {},{} ({}) were not adjacent, moving\n",
                        poses[i - 1].0,
                        poses[i - 1].1,
                        i - 1,
                        poses[i].0,
                        poses[i].1,
                        i
                    );
                    let h = poses[i - 1];
                    let t = poses[i];

                    print!("{},{} and {},{}  moving\n", h.0, h.1, t.0, t.1,);

                    if h.0 == t.0 {
                        if h.1 > t.1 {
                            poses[i].1 += 1;
                        } else if h.1 < t.1 {
                            poses[i].1 -= 1;
                        }
                    } else if h.1 == t.1 {
                        if h.0 > t.0 {
                            poses[i].0 += 1;
                        } else if h.0 < t.0 {
                            poses[i].0 -= 1;
                        }
                    } else {
                        if h.0 > t.0 {
                            if h.1 > t.1 {
                                poses[i].0 += 1;
                                poses[i].1 += 1;
                            } else {
                                poses[i].0 += 1;
                                poses[i].1 -= 1;
                            }
                        } else if h.0 < t.0 {
                            if h.1 > t.1 {
                                poses[i].0 -= 1;
                                poses[i].1 += 1;
                            } else {
                                poses[i].0 -= 1;
                                poses[i].1 -= 1;
                            }
                        }
                    }
                }
                print!(
                    "New: {},{} ({}) and {},{} ({})\n",
                    poses[i - 1].0,
                    poses[i - 1].1,
                    i - 1,
                    poses[i].0,
                    poses[i].1,
                    i
                );
            }
            print!("New coordinates {} {}\n", poses[0].0, poses[0].1);
            print!("tail coordinates {} {}\n", poses[9].0, poses[9].1);

            m.set((poses[9].0, poses[9].1), 1 as u8);

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

    for y in 750..850 {
        for x in 750..850 {
            if m.get((x, y)) > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    print!("Visited: {}\n", visited);
}

fn is_adjacent(t: (usize, usize), h: (usize, usize)) -> bool {
    print!("Checking adjacent {},{} and {},{}\n", h.0, h.1, t.0, t.1,);

    if h.0 == t.0 && h.1 == t.1 {
        print!("True - same\n");

        return true;
    }

    if h.0 > 0 && t.0 == (h.0 - 1) && (h.1 > 0 && t.1 == h.1 - 1 || t.1 == h.1 || t.1 == (h.1 + 1))
    {
        print!("True - 1\n");
        return true;
    }

    if t.0 == h.0 && (h.1 > 0 && t.1 == h.1 - 1 || t.1 == h.1 || t.1 == (h.1 + 1)) {
        print!("True - 2\n");
        return true;
    }

    if t.0 == (h.0 + 1) && (h.1 > 0 && t.1 == h.1 - 1 || t.1 == h.1 || t.1 == (h.1 + 1)) {
        print!("True - 3\n");

        return true;
    }

    return false;
}
