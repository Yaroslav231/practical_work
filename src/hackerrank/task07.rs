// https://www.hackerrank.com/challenges/between-two-sets/problem

use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut out: i32 = 0;

    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    for i in max_a..=min_b {
        let res1 = a.iter().all(|&num| i % num == 0);
        let res2 = b.iter().all(|&num| num % i == 0);
        if res1 && res2 {
            out += 1;
        }
    }
    out
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    stdin_iterator.next();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = get_total_x(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}

#[test]
fn test() {
    main()
}