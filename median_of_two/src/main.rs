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

    println!("{:?} {:?}", lena, lenb);
    println!("{:?} {:?}", veca, vecb);
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut steps: u32 = 0;
    let conlen = nums1.len() + nums2.len();

    if (conlen & 0b1) != 0 {
        let mut med1pos = nums1.len() / 2;
        let mut bottom1pos = 0;
        let mut top1pos = nums1.len() - 1;

        let mut med2pos = nums2.len() / 2;
        let mut bottom2pos = 0;
        let mut top2pos = nums2.len() - 1;


        while (true) {
            let n1 = *nums1.get(med1pos).unwrap_or(&0);
            let n2 = *nums2.get(med2pos).unwrap_or(&0);
            if med1pos + med2pos > nums1.len() {
                if(n1 > n2){
                    top1pos = med1pos;
                    med1pos = (med1pos + bottom1pos) / 2;
                }else if (n1 < n2){
                    top2pos = med2pos;
                    med2pos = (med2pos + bottom2pos) / 2;
                }else{
                    return n1 as f64;
                }
            }else{
                if n1 < n2 {
                    bottom1pos = med1pos;
                    med1pos = (med1pos + top1pos) / 2;
                } else if n1 > n2 {
                    bottom2pos = med2pos;
                    med2pos = (med2pos + top2pos) / 2;
                } else {
                    n1
                }
            }
            
        }
    }

    todo!()
}



struct binary_search_info {
    median_pos:usize,
    bottom_pos:usize,
    top_pos:usize
}

impl binary_search_info {
    fn new (bottom:usize, top:usize) -> Result<binary_search_info,String>{
        if top < bottom {
            return Err(String::from("Top cannot be smaller than bottom"));
        }
        

        todo!()
    }

}