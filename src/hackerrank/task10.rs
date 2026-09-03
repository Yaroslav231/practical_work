// https://www.hackerrank.com/challenges/sock-merchant/problem

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sock_merchant(ar: &[i32]) -> i32 {
    let mut pairs = 0;
    let mut socks = HashSet::new();

    for &i in ar {
        if socks.contains(&i) {
            socks.remove(&i);
            pairs += 1;
        } else {
            socks.insert(i);
        }
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();
    stdin_iterator.next();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}


#[test]
fn test() {
    main()
}