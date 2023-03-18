
use rand::Rng;

fn main() {

    let target:u32 = rand::random::<u32>() & (255 as u32);
    let numssize = 64;

    let nums: Vec<u32> = (0..numssize).map(|v| (rand::random::<u32>()+v*27) & (255 as u32)).collect();


    println!("{target} \n {}",nums.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ") );

    if let Some((i, j)) = find_indexes(target, &nums) {
        println!("Die Indizes sind {i} und {j}, mit den Werten {:?} und {:?}.", nums.get(i), nums.get(j));
    } else {
        println!("Es wurden keine passenden Zahlen in der Liste gefunden"); 
    }

    
}




fn find_indexes(target:u32, nums:& Vec<u32>) -> Option<(usize, usize)> {

    for i in 0 .. nums.len() {
        let num_one = match nums.get(i) { Some(n) if *n < target => *n, _ => continue,};
        for j in i..nums.len() {
            let num_two = match nums.get(j) { Some(n) if *n < target => *n, _ => continue,};
            if num_one + num_two == target{
                return Some((i,j));
            }
        }
    }
    None
}