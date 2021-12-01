use std::io;
use std::io::prelude::*;

fn main() {
    // there is an error introduced in this approach that we have to subtract
    let mut total_increased: i32 = -3;
    let mut scan: [i32; 3] = [0, 0, 0];
    let mut prev: i32 = -1;
    for (i, line) in io::stdin().lock().lines().enumerate() {
        let input = line.unwrap().parse::<i32>().unwrap();
        match i % 3 {
            0 => scan[0] = input,
            1 => scan[1] = input,
            2 => scan[2] = input,
            _ => (),
        }
        let arr_sum: i32 = scan.iter().sum::<i32>();
        if arr_sum > prev {
            total_increased += 1;
        }
        prev = arr_sum;
    }
    println!("{}", total_increased.to_string());
}
