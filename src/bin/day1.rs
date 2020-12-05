use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("./data/day1-input.txt");
    let display = path.display();

    let values = match File::open(&path) {
        Err(reason) =>  panic!("Failed to open input file {}: {}", display, reason),
        Ok(file) => io::BufReader::new(file)
                        .lines()
                        .map(|line| line.map(|s| s.parse::<u32>()))
                        .flatten()
                        .map(|res| match res {
                            Ok(v) => v,
                            Err(reason) => panic!(reason),
                        }),
    };

    // Only one phase can be executed at a time,
    // since iterator can be consumed only once.
    // Thus one is commented out.

    //let res_a = compare_a(values.collect());
    //println!("Result to first half is {}", res_a);

    let vv: Vec<u32> = values.collect();
    let res_b = compare_b(&vv[0..]);
    println!("Result to second half is {}", res_b);
}

// First phase, with vector
fn compare_a(mut list: Vec<u32>) -> u32 {
    let head = match list.pop() {
        Some(number) => number,
        None => panic!("No value at first index, end of list without match found."),
    };

    for elem in &list {
        if head + elem == 2020 {
            println!("Match with values {}, {}", head, elem);
            return head * elem;
        }
    }
    return compare_a(list);
}

// Second phase, with arrays / slices
fn compare_b(values: &[u32]) -> u32 {

    fn comp(a: u32, rest: &[u32]) -> Option<(u32, u32)> {

        fn comp2(a: u32, b: u32, rest: &[u32]) -> Option<u32> {
            if rest.is_empty() {
                return None;
            }

            let c = rest[0];
            let tail = &rest[1..];

            if a + b + c == 2020 {
                println!("Match with values {}, {}, {}", a, b, c);
                return Some(c)
            }
            return comp2(a, b, tail);
        }

        if rest.is_empty() {
            return None
        }

        let second = rest[0];
        let tail = &rest[1..];

        return match comp2(a, second, tail) {
            Some(third) => Some((second, third)),
            None => comp(a, tail),
        }
    }

    let first = values[0];
    let tail = &values[1..];

    return match comp(first, tail) {
        Some((second, third)) => first * second * third,
        None => compare_b(tail),
    }
}
