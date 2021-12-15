use std::io;
use std::io::prelude::*;

// TODO: some function that will check if number are done

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();

    // after this, the first line is removed from the iterator
    let first_line = lines_iter.next().unwrap().unwrap();
    let drafts: Vec<i32> = first_line
        .split(",")
        .map(|x| x.trim())
        .filter(|s| !s.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let mut i = 0;
    // there is one more row and column for checking if number was chosen
    let mut matrices: Vec<[[i32; 6]; 6]> = Vec::new();
    let mut new_matrix: [[i32; 6]; 6] = [[0; 6]; 6];
    for unwrapped_line in lines_iter {
        let line = unwrapped_line.unwrap();
        if line.is_empty() {
            continue
        }

        for (j, num) in line
            .split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .enumerate() {
            new_matrix[(i%5) as usize][j] = num;
        }

        if i != 0 && i % 5 == 0 {
            matrices.push(new_matrix);
            new_matrix = [[0; 6]; 6];
        }

        i += 1;
    }
    for matrix in matrices.iter() {
        for line in matrix {
            for num in line {
                print!("{} ", num);
            }
            println!("");
        }
        println!("");
    }
    const CHECK: usize = 5;
    for num in drafts {
        for matrix in &mut matrices {
            // make this a function
            let mut i: usize = 0;
            for row in *matrix {
                if i == 5 {
                    break;
                }
                for (j, guess) in row.iter().enumerate() {
                    if j == 5 {
                        break;
                    }
                    if guess == &num {
                        matrix[i][CHECK] += 1;
                        matrix[CHECK][j] += 1;
                        // I could break at this point, but I dont know the
                        // rules of bingo
                        if matrix[i][CHECK] == 5 {
                            let row_sum: i32 = matrix[i].iter().sum();
                            println!("{}", row_sum * num);
                            return
                        }
                        if matrix[CHECK][j] == 5 {
                            let mut row_sum = 0;
                            for k in 0..4 {
                                row_sum += matrix[k][j];
                                println!("{}", row_sum);
                            }
                            println!("{}", row_sum * num);
                            return
                        }
                    }
                }
                i += 1;
            }
        }
    }
}
