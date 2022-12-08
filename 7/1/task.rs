use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let mut pwd = Vec::<&str>::new();
    let mut sizes = HashMap::<String, i32>::new();

    pwd.push("/");

    for i in 0..lines.len() {
        let line = lines[i].as_str();
        let v: Vec<&str> = line.split_whitespace().collect();

        if v[0] == "$" {
            if v[1] == "cd" {
                if v[2] == ".." {
                    pwd.truncate(pwd.len() - 1);
                } else if &v[2][0..0] == "/" {
                    pwd.clear();
                    pwd.push(v[2]);
                } else {
                    pwd.push(v[2]);
                }
            }
        } else if v[0] != "dir" {
            let mut fullname = pwd.to_owned();
            fullname.push(v[1]);
            let k = fullname.join("/");
            sizes.insert(k, v[0].parse::<i32>().unwrap());
        }
    }

    let mut dir_sizes = HashMap::<String, i32>::new();
    for p in sizes.keys() {
        let dir = p[0..p.rfind("/").unwrap()].to_string().to_owned();
        while dir.len() > 0 {
            if dir_sizes.contains_key(&dir) {
                dir_sizes.insert(dir, sizes[p] + dir_sizes[&dir]);
            } else {
                dir_sizes.insert(dir, sizes[p]);
            }
        }
    }

    print!("\n\n");

    print!("\n\n ");
}
