use std::io;
use std::io::prelude::*;
use std::convert::TryInto;

const BINARY_ARR_SIZE: usize = 12;

fn main() {
    let mut binary: [u32; BINARY_ARR_SIZE] = [0; BINARY_ARR_SIZE];
    let mut i: usize = 0;
    for line in io::stdin().lock().lines() {
        for (j, num) in line.unwrap().chars().enumerate() {
            if num == '1' {
                binary[j] += 1;
            }
        }
        i += 1;
    }
    let mut tmp: u32 = 0;
    let mid = i / 2;
    for (j, num) in binary.iter().enumerate() {
        if *num > mid.try_into().unwrap() {
            tmp += 2_u32.pow((BINARY_ARR_SIZE - j).try_into().unwrap());
        }
    }
    println!("{:#b}\n{:#b}", tmp, ((!tmp << 21) >> 21));
    let res: u64 = (tmp * ((!tmp << 21) >> 21)).into();
    println!("{} * {} = {}", tmp, ((!tmp << 21) >> 21), res);
}
