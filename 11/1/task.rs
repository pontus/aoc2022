use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let mut monkey_items = Vec::new();
    let mut ops = Vec::new();
    let mut dest = Vec::new();
    let mut inspect = Vec::new();

    let monkeys = lines.len() / 7 + 1;
    let mut i = 0;

    while i < monkeys {
        monkey_items.push(Vec::new());
        ops.push(Vec::new());
        dest.push(Vec::new());
        inspect.push(0);
        i += 1;
    }

    i = 0;
    while i < monkeys {
        let item_line = &lines[i * 7 + 1];
        let mut s = item_line.split(" ").skip(4);

        let mut item = s.next();
        while item != None {
            let mut num_str = item.unwrap();
            if num_str.find(",") != None {
                num_str = num_str.get(..num_str.find(",").unwrap()).unwrap();
            }

            let to = num_str.parse::<i32>().unwrap();
            // let to = 4;
            monkey_items[i].push(to);
            item = s.next();
        }

        let op_line = &lines[i * 7 + 2];
        s = op_line.split(" ").skip(6);

        ops[i].push(-1000);
        let op = s.next().unwrap();

        match op {
            "+" => ops[i].push(1),
            "-" => ops[i].push(2),
            "*" => ops[i].push(3),
            "/" => ops[i].push(4),
            _ => print!("Something wrong"),
        }

        let value = s.next().unwrap();

        if value == "old" {
            ops[i].push(-1000);
        } else {
            ops[i].push(value.parse::<i32>().unwrap());
        }

        let test_line = &lines[i * 7 + 3];
        s = test_line.split(" ").skip(5);
        let f = s.next().unwrap();

        dest[i].push(f.parse::<usize>().unwrap());

        let true_line = &lines[i * 7 + 4];
        s = true_line.split(" ").skip(9);
        dest[i].push(s.next().unwrap().parse::<usize>().unwrap());

        let false_line = &lines[i * 7 + 5];
        s = false_line.split(" ").skip(9);
        dest[i].push(s.next().unwrap().parse::<usize>().unwrap());

        i += 1;
    }

    println!("{:?}", monkey_items);
    println!("{:?}", ops);

    println!("{:?}", dest);

    println!("\n\nRounds\n\n");

    for _round in 0..20 {
        println!("{:?}", monkey_items);

        for monkey in 0..monkeys {
            for i in 0..monkey_items[monkey].len() {
                let mut item = monkey_items[monkey].pop().unwrap();
                inspect[monkey] += 1;

                let value = match ops[monkey][2] {
                    -1000 => item,
                    _ => ops[monkey][2],
                };

                let new = match ops[monkey][1] {
                    1 => item + value,
                    2 => item - value,
                    3 => item * value,
                    4 => item / value,
                    _ => {
                        println!("WRONG!");
                        -1
                    }
                } / 3;

                if new % (dest[monkey][0] as i32) == 0 {
                    monkey_items[dest[monkey][1]].push(new);
                } else {
                    monkey_items[dest[monkey][2]].push(new);
                }
            }
        }
    }

    println!("{:?}", monkey_items);
    println!("{:?}", ops);

    println!("{:?}", dest);

    println!("{:?}", inspect);
    inspect.sort();
    inspect.reverse();
    println!("{:?}", inspect[0] * inspect[1]);
}
