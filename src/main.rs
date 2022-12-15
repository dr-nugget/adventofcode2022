use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

fn main() {
    let day1_ans = day1();
    println!("Day 1 answer 1:");
    println!("{}", day1_ans.0);
    println!("Day 1 answer 2:");
    println!("{}\n", day1_ans.1);

    let day2_ans = day2();
    println!("Day 2 answer 1:");
    println!("{}", day2_ans.0);
    println!("Day 2 answer 2:");
    println!("{}\n", day2_ans.1);
}

#[allow(unused_assignments)]
fn day1() -> (u32, u32) {
    let input_file = get_file(1);
    let mut reader = BufReader::new(input_file);
    let mut total: u32 = 0;
    let mut totals: Vec<u32> = Vec::new();

    with_each_line(&mut reader, |buf| {
        if buf == "\n" {
            totals.push(total);
            total = 0;
        } else {
            let buf = buf.strip_suffix("\n").unwrap().to_string();
            total += buf.parse::<u32>().unwrap();
        }
    });

    let max = **(&totals.iter().max().unwrap());

    // Use the std max-order BinaryHeap to get 3 highest values
    let mut heap = BinaryHeap::new();
    for val in totals {
        heap.push(val);
    }
    let mut top_three = Vec::new();
    for _ in 0..3 {
        top_three.push(heap.pop().unwrap());
    }
    let top_three_sum: u32 = top_three.iter().sum();

    (max, top_three_sum)
}

fn day2() -> (u16, u16) {
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let win = 6;
    let tie = 3;
    let loss = 0;

    let mut reader = BufReader::new(get_file(2));
    let mut score1: u16 = 0;
    with_each_line(&mut reader, |buf| {
        let buf = &mut buf.strip_suffix("\n").unwrap().to_string();
        let letters: Vec<&str> = buf.split(' ').collect();
        let mut vals = Vec::new();
        for letter in letters {
            let val = match letter {
                "A" | "X" => rock,
                "B" | "Y" => paper,
                "C" | "Z" => scissors,
                _ => panic!("Letter encountered that's not A-C or X-Z")
            };

            vals.push(val);
        }

        let my_move = vals[1];
        let their_move = vals[0];

        score1 += my_move + if my_move == rock {
            if their_move == rock {
                tie
            } else if their_move == paper {
                loss
            } else {
                win
            }
        } else if my_move == paper {
            if their_move == rock {
                win
            } else if their_move == paper {
                tie
            } else {
                loss
            }
        } else {
            if their_move == rock {
                loss
            } else if their_move == paper {
                win
            } else {
                tie
            }
        }
    });

    let mut score2: u16 = 0;
    _ = reader.seek(SeekFrom::Start(0));
    with_each_line(&mut reader, |buf| {
        let buf = &mut buf.strip_suffix("\n").unwrap().to_string();
        let letters: Vec<&str> = buf.split(' ').collect();
        let mut vals = Vec::new();
        for letter in letters {
            let val = match letter {
                "A"=> rock,
                "B"=> paper,
                "C"=> scissors,
                "X" => loss,
                "Y" => tie,
                "Z" => win,
                _ => panic!("Letter encountered that's not A-C or X-Z")
            };

            vals.push(val);
        }

        let outcome = vals[1];
        let their_move = vals[0];

        score2 += outcome + if outcome == loss {
            if their_move == rock {
                scissors
            } else if their_move == paper {
                rock
            } else {
                paper
            }
        } else if outcome == tie {
            if their_move == rock {
                rock
            } else if their_move == paper {
                paper
            } else {
                scissors
            }
        } else {
            if their_move == rock {
                paper
            } else if their_move == paper {
                scissors
            } else {
                rock
            }
        }
    });

    (score1, score2)
}

fn get_file(day: u8) -> File {
    let path = format!("./assets/aoc{}.txt", day);
    File::open(path).expect(&format!("No input file for day {}", day))
}

fn with_each_line<F>(reader: &mut BufReader<File>, mut func: F)
where
    F: FnMut(&mut String),
{
    loop {
        let buf = &mut String::new();
        let bytes_read = reader.read_line(buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        func(buf);
    }
}
