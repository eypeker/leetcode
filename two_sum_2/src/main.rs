fn main() {
    println!("Hello, world!");
}




impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() -1{
            let n = nums[i];
            for j in (i + 1 .. nums.len()){
                let m = nums[j];
                if m+n == target {
                    return vec![i as i32,j as i32];
                }
            }
        }
        return vec![-1,-1];
    }
}