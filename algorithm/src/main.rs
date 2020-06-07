// #![feature(bool_to_option)]
mod a2;
mod a64;
mod a837;


struct Solution;

fn main() {
    // println!("res: {:?}", Solution::sum_nums(3));
    let x: usize = 10;
    for i in (0..=x).rev(){
        println!("{}", i);
    }

    println!("solution:{}", Solution::new21_game(10,1, 10));
}
