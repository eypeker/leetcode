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
    //println!("{}", find_median_sorted_arrays(veca, vecb));
}

enum direction {
    not_chosen,
    left_to_right,
    right_to_left,
}


pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let conlen = nums1.len() + nums2.len();
    let medianindex = (conlen-1) / 2;
    let mut indexleft = 0; 
    let mut indexright = 0;
    let mut dir = direction::not_chosen;
    if nums1.len() < medianindex / 2  {
        indexleft = nums1.len() -1;
        indexright = medianindex -indexleft; 
    }else if nums2.len() < medianindex /2 {
        indexright = nums2.len();
        indexleft = medianindex - indexright;
    }else {
        indexleft = conlen / 2;
        indexright = conlen - indexleft;
    }

    match (nums1.get(indexleft), nums2.get(indexright)) {
        (Some(a), Some(b)) if *a < *b => dir = direction::right_to_left,
        (Some(a), Some(b)) if *a > *b => dir = direction::left_to_right,
        (Some(a), Some(b)) if *a == *b => return *a as f64,
        (Some(a),None) => return *a as f64,
        (None, Some(b)) => return *b as f64,
        _ => (),
    }

    match dir {
        direction::left_to_right => (),
        direction::right_to_left =>(),
        direction::not_chosen =>(),
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
