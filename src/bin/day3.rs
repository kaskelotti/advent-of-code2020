use std::fs::File;
use std::io::{self, BufRead};

struct TobogganRide {
    hits: usize,
    pos: usize,
}

fn main() -> io::Result<()> {
    let init = TobogganRide {
        hits: 0,
        pos: 0
    };

    let file = File::open("./data/day3-input.txt")?;
    let ride = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .fold(init, |acc, current_row| {
            let TobogganRide { hits, pos } = acc;

            let ch = match current_row.get(pos..pos + 1) {
                Some(value) => value,
                None => panic!("Something went wrong here at position {}", pos)
            };

            let is_hit = ch == "#";

            TobogganRide {
                hits: if is_hit { hits + 1 } else { hits },
                pos: {
                    let len = current_row.len();
                    {
                        let pos1 = pos + 3;
                        if pos1 >= len {
                            pos1 - len
                        } else {
                            pos1
                        }
                    }
                }
            }
        });

    println!("Trees hit: {}", ride.hits);

    Ok(())
}
