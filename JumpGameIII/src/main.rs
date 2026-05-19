fn main() {
    
}

struct Solution;


impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut copy = arr;
        Solution::can_reach_rec(copy.as_mut(), start)

    }

    fn can_reach_rec(arr: & mut Vec<i32>, start: i32) -> bool{
        let value = arr[start as usize];
        let length = arr.len();
        if value < 0 { // look up if this index is used in this walkthrough
            return false;
        }
        arr[start as usize] = -value; // mark an index as beeing used in this walkthrough
        if value == 0{
            return true;
        }
        if start >= value &&  Solution::can_reach_rec(arr, start - value){
            return true;
        }
        if (start+value) < length as i32 && Solution::can_reach_rec(arr, start + value){
            return true;
        }
        arr[start as usize] = value;
        return false;
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_one() {
        let vec_one = vec![4,2,3,0,3,1,2];
        let start = 5;
        assert_eq!(Solution::can_reach(vec_one,start),true);
    }

    #[test]
    fn test_two() {
        let vec_two = vec![4,2,3,0,3,1,2];
        let start = 0;
        assert_eq!(Solution::can_reach(vec_two,start),true);
    }

    #[test]
    fn test_three(){
        let vec_three = vec![3,0,2,1,2];
        let start = 2;
        assert_eq!(Solution::can_reach(vec_three, start),false);
    }

}