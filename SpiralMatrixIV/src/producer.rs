use rand::Rng;
use std::collections::LinkedList;

pub fn produce_list(max_size:u32) -> LinkedList<u32>{
    let mut rng = rand::thread_rng();
    let vecsize = rng.gen_range(0..max_size);
    let vec = (0..vecsize).map(|_| rng.gen_range(0..1000) as u32).collect::<LinkedList<u32>>();
    vec
}
pub fn gen_rand_size() -> (u32,u32){
    let mut rng = rand::thread_rng();
    let m = rng.gen_range(0..1001) as u32;
    let n = match rng.gen_range(0..100001) as u32{
        x if  m * x > 100000 => 100000 / m,
        x => x
    };
    (m,n)
}