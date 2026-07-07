fn main() {
}



impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut highest = 0;
        let mut current_height=0;

        for n in gain.into_iter() {
            current_height += n;
            if current_height > highest{
                highest = current_height;
            }
        }
        highest
    }
}