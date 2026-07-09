//https://leetcode.com/problems/path-existence-queries-in-a-graph-i/description/?envType=daily-question&envId=2026-07-09

fn main() {
    println!("Hello, world!");
}


impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut answers = vec![false; queries.len()];
        for (querynumber,query) in queries.into_iter().enumerate(){
            let (first,last) = match (query[0] as usize ,query[1] as usize)  {
                (a,b) if a <= b => (a,b),
                (a,b) if a > b => (b,a),
                _ =>panic!("Either a should be bigger or b, there is no other way")
            };
            
            let mut runnerindex = first;
            loop {
                if runnerindex == last {
                    answers[querynumber] = true;
                    break;
                }else if nums[runnerindex] + max_diff < nums[runnerindex+1]{
                    break;
                }
                runnerindex += 1;
            }
        }
        answers
    }
}