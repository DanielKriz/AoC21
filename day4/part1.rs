use std::io;
use std::io::prelude::*;

/// TODO: some function that will check if number are done

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();
    ///
    /// after this, the first line is removed from the iterator
    let first_line = lines_iter.next().unwrap().unwrap();
    let drafts: Vec<i32> = first_line
        .split(",")
        .map(|x| x.trim())
        .filter(|s| !s.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let mut i = 0;
    let mut matrices: Vec<[[i32; 5]; 5]>;
    for unwrapped_line in lines_iter {
        let line = unwrapped_line.unwrap();
        if line.is_empty() {
            continue
        }

        if i == 5 {
            i = 0;
        }

        i += 1;
        /// fill the matrices vector
        println!("{}", line);
    }
}
