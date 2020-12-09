use std::fs::File;
use std::io::{self, BufRead};

struct Requirement {
    min: usize,
    max: usize,
    char: String,
}

struct Password {
    text: String,
    req: Requirement,
}

fn parse_password(ss: String) -> Password {
    let find_index = |ch| ss.find(ch).unwrap();
    let parse_from = |a, b| {
        return match ss.get(a..b) {
            Some(value) => value.parse::<usize>().unwrap(),
            None => panic!("Cannot parse {} at {}..{}", ss, a, b),
        };
    };

    let pos1 = find_index('-');
    let pos2 = find_index(' ');
    let pos3 = find_index(':');

    let min = parse_from(0, pos1);
    let max = parse_from(pos1 + 1, pos2);
    let char = ss.get(pos2 + 1..pos3).unwrap();
    let text = ss.get(pos3 + 2..).unwrap();

    return Password {
        text: text.to_string(),
        req: Requirement {
            min: min,
            max: max,
            char: char.to_string()
        }
    };
}

fn main() -> io::Result<()> {
    let is_valid = |password: &Password| {
        let count = password.text.char_indices()
            .filter(|(_, char)| char.to_string() == password.req.char)
            .count();

        return count >= password.req.min && count <= password.req.max;
    };

    let file = File::open("./data/day2-input.txt")?;
    let count = io::BufReader::new(file)
        .lines()
        .map(|line| parse_password(line.unwrap()))
        .filter(|password| is_valid(password))
        .count();

    println!("count {}", count);

    Ok(())
}
