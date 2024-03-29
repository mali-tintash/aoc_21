mod challenges;

use challenges::chal1;

use challenges::chal2;
use challenges::day2;
use challenges::day3;

fn main() {
    println!("Welcome to AOC 21");
    let path = "./chal1_input.txt";
    let big_count = chal1::larger_measurements_count(path);
    println!("challenge 1: {}\n", big_count);
    println!("challenge 2: {}\n", chal2::larger_window_count(path));

    println!("Day2 (A): {}\n",day2::calculate_pos_depth());
    println!("Day2 (B): {}\n",day2::calculate_pos_depth_aim());

    challenges::day3::calculate_gama_epsilon();
}
