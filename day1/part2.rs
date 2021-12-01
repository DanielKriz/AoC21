use std::io;
use std::io::prelude::*;
use std::convert::TryInto;

fn main() {
    let mut total_increased: i32 = -1;
    let mut i: u32 = 0;
    let mut windows: [i32;4] = [-1, -1, -1, -1];
    let mut prev: i32 = -1;
    for line in io::stdin().lock().lines() {
        i += 1;
        let input = line.unwrap().parse::<i32>().unwrap();
        if i % 4 == 1 {
            windows[0] += input;
            windows[2] += input;
            windows[3] += input;
            if windows[2] > prev {
                total_increased += 1;
            }
        } else if i % 4 == 2 {
            windows[0] += input;
            windows[1] += input;
            windows[3] += input;
            if windows[3] > prev {
                total_increased += 1;
            }
        } else if i % 4 == 3 {
            windows[0] += input;
            windows[1] += input;
            windows[2] += input;
            if windows[0] > prev {
                total_increased += 1;
            }
        } else {
            windows[1] += input;
            windows[2] += input;
            windows[3] += input;
            if windows[1] > prev {
                total_increased += 1;
            }
        }
        let idx: usize = ((i + 1) % 4).try_into().unwrap();
        prev = windows[idx];
        windows[idx] = 0;
    }
    // there is an error introduced in this approach that we have to subtract
    total_increased -= 2;
    println!("{}", total_increased.to_string());
}
