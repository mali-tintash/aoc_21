mod challenges;

use challenges::chal1;

fn main() {
    println!("Welcome to AOC 21");

    let bigCount = chal1::larger_measurements_count(String::from("./chal1_input.txt"));
    println!("challenge 1: {}\n", bigCount);
}
