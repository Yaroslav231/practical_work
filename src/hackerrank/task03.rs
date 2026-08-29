// https://www.hackerrank.com/challenges/staircase/problem

use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    let current: i32 = n.clamp(1, 100);

    for i in 1..=current {
        let number_of_gratings: i32 = i;
        let number_of_spaces: i32 = current - i;
        println!("{}{}", " ".repeat(number_of_spaces as usize), "#".repeat(number_of_gratings as usize));
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}


#[test]
fn test() {
    main()
}
