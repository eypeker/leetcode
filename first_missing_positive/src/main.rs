



fn main() {
    
    let values = [3,4,2,-1,1];
    let res = first_missing_positive(&values);

    println!("{res}");

}

fn first_missing_positive(values: &[i32]) -> i32 {
    let mut borders= [0; 5];

    for i in 0..values.len(){
        match values[i] {
            a if a < 1 || a > values.len() as i32 => continue,
            a if a >=1 && a <= values.len() as i32 => borders[i] = a,
            _ => (),
        }
    }
    for i in 0..borders.len() {
        if borders[i] == 0 {
            return i as i32;
        }
    }
    (borders.len() +1) as i32

}








