use rand::Rng;
use std::vec::Vec;

pub fn producelist()-> Vec<i32>{
    let mut rng = rand::thread_rng();
    let listsize = rand_number_to_hundred_k();
    (0..listsize).map(|_|rng.gen_range(-10000..10001)).collect()
}


pub fn rand_number_to_hundred_k() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..100)
}