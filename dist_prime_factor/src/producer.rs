use rand::Rng;
use std::vec::Vec;

pub fn producelist() -> Vec<u32>{
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(0..1000);
    (0..length).map(|i|{
        rng.gen_range(2..1000)
    }).collect()
 }