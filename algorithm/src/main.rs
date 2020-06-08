// #![feature(bool_to_option)]
mod a2;
mod a3;
mod a64;
mod a837;


struct Solution;

fn main() {
    // println!("res: {:?}", Solution::sum_nums(3));
    // println!("solution:{}", Solution::new21_game(10,1, 10));
    println!("solution: {}", Solution::length_of_longest_substring(String::from("abcabcbb")));
}
