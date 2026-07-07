use std::iter;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut index1 = 0;
        let mut index2 = 0;
        let vec_len1 = nums1.len();
        let veclen2 = nums2.len();
        while index1 < vec_len1 && index2 < veclen2{
            let val1 = nums1[index1];
            let val2 = nums2[index2];
            if val1 == val2 {return val1;}
            let increase = val1 < val2;
            index1 = index1 + increase as usize;
            index2 = index2 +  (!increase) as usize;
        }
        -1
    }
}


#[cfg(test)]
mod test {
    use crate::Solution;


#[test]
fn test_one(){
    let vec1 = vec![1,2,3];
    let vec2 = vec![2,4];
    assert_eq!(Solution::get_common(vec1, vec2), 2);
}
#[test]
fn test_two(){
    let vec1 = vec![1,2,3,6];
    let vec2 = vec![2,3,4,5];
    assert_eq!(Solution::get_common(vec1, vec2), 2);
}

}