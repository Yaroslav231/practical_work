// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem

use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut min_count: i32 = 0;
    let mut max_count: i32 = 0;
    let mut current_min: i32 = scores[0];
    let mut current_max: i32 = scores[0];

    for &score in scores {
        if score < current_min {
            current_min = score;
            min_count += 1;
        }
        if score > current_max {
            current_max = score;
            max_count += 1;
        }
    }

    vec![max_count, min_count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();
    stdin_iterator.next();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[test]
fn test() {
    main()
}