use std::collections::LinkedList;
mod producer;

fn main() {
    let (m,n) = producer::gen_rand_size();
    let random_list = producer::produce_list(m*n);
    //println!("{:?}",random_list);
    let matrix = get_matrix(random_list,m,n);
    for i in 0..n{
        for j in 0..m {
            print!("{}\t\t", matrix.get((i*m +j) as usize).unwrap_or(&-2) );
        }
        println!();
    }
    //println!("{:?}", matrix);
}

fn get_matrix(list_of_values:LinkedList<u32>, m:u32, n:u32) -> Vec<i32>{
    let mut matrix = vec![-1; (m * n) as usize];
    let mut counter = 0;
    let mut index:usize = 0;
    let mut width = m+1;
    let mut height = n;
    let mut direction = Direction::Right;
    let _ = list_of_values.iter().for_each(|value| {
        matrix[index] = *value as i32;
        counter = counter +1;
        direction = match (direction, width, height, counter) {
            (Direction::Right,w,_,c) if c == w -1 => {width = width -1; counter = 0; Direction::Down},
            (Direction::Down, _,h,c) if c == h -1 => {height = height -1; counter = 0; Direction::Left},
            (Direction::Left,w,_,c) if c == w-1 => {width = width -1; counter = 0; Direction::Up},
            (Direction::Up,_,h,c) if c == h-1 => {height = height -1; counter = 0; Direction::Right},
            (d,_,_,_)=> d
        };
        match direction {
            Direction::Right => index = index + 1,
            Direction::Down => index = index + m as usize,
            Direction::Left => index = index -1,
            Direction::Up => index = index - m as usize
        }
        matrix[index] = *value as i32;
    });
    matrix
}

#[derive(Copy,Clone)]
enum Direction {
    Right,Down, Left, Up
}
