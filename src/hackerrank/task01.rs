// https://www.hackerrank.com/challenges/simple-array-sum/problem

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let sum: i32 = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Not a number"))
        .sum();

    println!("{}", sum);
}