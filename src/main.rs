use clap::Parser;
use hashmap_count::{Mode, Opts};
use std::collections::HashMap;

/*
This example code counts the frequency of each number in the vector.
 */

fn logic(numbers: &[i64]) -> Vec<(i64, u64)> {
    let mut frequencies = HashMap::new();

    for &num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    frequencies.into_iter().collect()
}

fn print_mode(result: &Vec<(i64, u64)>) {
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut result;

    match opts.mode {
        Mode::NormalMode => {
            let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
            result = logic(&numbers);
        }
        Mode::UserListMode => {
            let numbers = opts.numbers.unwrap_or_default();
            result = logic(&numbers);
        }
    }

    print_mode(&result);

    result.sort_by(|a, b| b.1.cmp(&a.1));
    print_mode(&result);

}
