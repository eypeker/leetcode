use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();

    let lena: usize = rng.gen_range(2..10);
    let lenb: usize = rng.gen_range(2..10);
    let veca = {
        let mut k: Vec<i16> = (0..lena).map(|_| rng.gen()).collect();
        k.sort();
        k
    };
    let vecb = {
        let mut k: Vec<i16> = (0..lenb).map(|_| rng.gen()).collect();
        k.sort();
        k
    };

    let veca = vec![1,5,7,10];
    let vecb = vec![2,3,9];

    println!("{:?} {:?}", lena, lenb);
    println!("{:?} {:?}", veca, vecb);
    println!("{}", find_median_sorted_arrays(veca, vecb));
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut steps: u32 = 0;
    let conlen = nums1.len() + nums2.len();

    if (conlen & 0b1) != 0 {
        let mut b1 = binary_search_info::new(0,nums1.len()-1).unwrap();
        let mut b2 = binary_search_info::new(0,nums2.len()-1).unwrap();

        loop {
            let n1 = *nums1.get(b1.median_pos).unwrap_or(&0);
            let n2 = *nums2.get(b2.median_pos).unwrap_or(&0);

            
            steps += 1;
            println!("{:?} {:?}",b1, b2);
        }

    }

    0.0
}
#[derive(Debug)]
struct binary_search_info {
    bottom_pos: usize,
    median_pos: usize,
    top_pos: usize,
}

impl binary_search_info {
    fn new(bottom: usize, top: usize) -> Result<binary_search_info, &'static str> {
        if top < bottom {
            return Err("Top cannot be lower than Bottom");
        }
        let median = (bottom + top) / 2;
        Ok(binary_search_info {
            bottom_pos: bottom,
            median_pos: median,
            top_pos: top,
        })
    }

    fn upper_half(&mut self) {
        self.bottom_pos = self.median_pos;
        self.median_pos = (self.median_pos + self.top_pos + 1) / 2;
    }

    fn lower_half(&mut self) {
        self.top_pos = self.median_pos;
        self.median_pos = (self.bottom_pos + self.median_pos) / 2;
    }
}
