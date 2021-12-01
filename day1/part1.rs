use std::io;
use std::io::prelude::*;

fn main() {
    let mut prev: i32 = 99999;
    let mut total_increased: i32 = 0;
    let mut curr: i32;
    for line in io::stdin().lock().lines() {
        curr = line.unwrap().parse::<i32>().unwrap();
        if curr > prev {
            total_increased += 1;
        }
        prev = curr;
    }
    println!("{}", total_increased.to_string());
}
