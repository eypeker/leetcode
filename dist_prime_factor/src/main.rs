mod producer;

fn main() {
    let mut nums = producer::producelist();
    let mut primes = vec![];


    let result = nums.iter_mut().map(|n|{
        let mut vec = vec![];
        let mut i = 2;
        while i <= (*n)/2{
            if *n % i == 0{
                while *n % i == 0 {
                    *n /= i;
                }
                vec.push(i);
            }
            i += 1;
        }
        vec
    }).flatten()
    .collect::<Vec<u32>>();


    let mut show = result.clone();
    result.iter().for_each(|v|{
        if !primes.contains(v) {
            primes.push(*v);
            show = show.iter().filter(|value|!(**value == *v)).map(|f| *f).collect::<Vec<u32>>();
            println!("{:?}\n\n",show);
        }

    });

    println!("{}", result.len());
    println!("{:?}", result);
    println!("{}", primes.len());
    println!("{:?}", primes);

}
