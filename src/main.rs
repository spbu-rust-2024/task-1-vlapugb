use ::itertools::Itertools;
use std::io;

fn insert_sort(s: String) -> Vec<i32> {
    let mut numbers: Vec<i32> = s
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    for i in 1..numbers.len() {
        let mut j = i;
        while j > 0 && numbers[j - 1] > numbers[j] {
            numbers.swap(j, j - 1);
            j -= 1;
        }
    }
    return numbers;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    println!("{}", Itertools::join(&mut insert_sort(input).iter(), " "));
}
