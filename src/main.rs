use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let day1_ans = day1();
    println!("Day 1 answer 1:");
    println!("{}\n", day1_ans.0);
    println!("Day 2 answer 2:");
    println!("{}\n", day1_ans.1);
}

#[allow(unused_assignments)]
fn day1() -> (u32, u32) {
    let input_file = get_file(1);
    let mut reader = BufReader::new(input_file);
    let mut total: u32 = 0;
    let mut totals: Vec<u32> = Vec::new();

    loop {
        let mut buf = String::new();

        let byte_marker = reader.read_line(&mut buf).unwrap();
        if byte_marker == 0 { break; }
        
        
        if buf == "\n" {
            totals.push(total);
            total = 0;
        } else {
            buf = buf.strip_suffix("\n").unwrap().to_string();
            total += buf.parse::<u32>().unwrap();
        }
    }
    
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

    (max,top_three_sum)
}

fn get_file(day: u8) -> File {
    let path = format!("./assets/aoc{}.txt", day);
    File::open(path).expect(&format!("No input file for day {}", day))
}