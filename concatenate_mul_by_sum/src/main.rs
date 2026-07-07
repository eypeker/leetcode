fn main() {
    println!("Hello, world!");
}


pub enum Solution{}


impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut f = n;
        let mut sum = 0;
        let mut new_number = 0;
        let mut digits_multiplier = 1;
        while f > 0 {
            let  digit = f%10;
            sum += digit;
            f = f/10;
            if digit != 0 {
                new_number += digit * digits_multiplier;
                digits_multiplier *= 10;
            }
        }
        new_number as i64 * sum as i64
    }
}