use std::collections::HashMap;
// https://www.hackerrank.com/challenges/migratory-birds/problem
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts_map: HashMap<i32, i32> = HashMap::new();

    for &i in arr {
        counts_map.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut result = 0;
    let mut max_count = 0;
    for (&i, &count) in &counts_map {
        if (count > max_count) || (count == max_count && i < result) {
            max_count = count;
            result = i;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("output.txt").unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn test() {
    main()
}