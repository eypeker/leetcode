fn main() {
    println!("Hello, world!");
}


const hour_to_angle:f64 = 30.0;

const minute_to_angle:f64 = 6.0;

const minute_to_hour:f64 =0.5;


impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let minute_angle = minutes as f64 * minute_to_angle;
        let hour_angle = {
            let min_angle = hour as f64 * hour_to_angle;
            let added_angle = minute_to_hour * minutes as f64;
            min_angle + added_angle
        };
        if hour_angle > minute_angle {
            f64::min(hour_angle - minute_angle, minute_angle - hour_angle + 360.0)
        }else {
            f64::min(minute_angle - hour_angle, hour_angle - minute_angle + 360.0)
        }

    }
}