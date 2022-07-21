mod challenges;

use challenges::chal1;

use challenges::chal2;

fn main() {
    println!("Welcome to AOC 21");
    let path = "./chal1_input.txt";
    let big_count = chal1::larger_measurements_count(path);
    println!("challenge 1: {}\n", big_count);
    println!("challenge 2: {}\n", chal2::larger_window_count(path));
}
