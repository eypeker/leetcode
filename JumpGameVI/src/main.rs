use std::{cell::RefCell, cmp::min, fmt::Binary};

use producer::{producelist, rand_number_to_hundred_k};

mod producer;

fn main() {
    //test::learning()
    let k = rand_number_to_hundred_k();
    let list = producelist();
    println!("{}", !3);
    println!("k {}\t\t list {}",k,list.len());
    println!("Listenwerte {:?}",list );
    let bmp = get_best_path( k, list);
    println!("{:?}",bmp.data.iter()
                            .map(|u| vec![(*u) >> 24, (*u) >> 16, (*u) >> 8, *u ].iter().map(|b| *b as u8).collect::<Vec<u8>>()
                        ).flatten()); 
}


fn get_best_path(k:u32, list:Vec<i32> ) -> Bitmap{
    let mut bmp = Bitmap::new(list.len());
    bmp.set(0);
    let mut steps_to_skip:Skipsteps = Skipsteps::NoJump;
    list[1..list.len()-1].iter().enumerate().for_each(|i|{
        match i {
            (index, value) if *value >= 0 => bmp.set(index),
            (index, value) if *value < 0 => {
                match steps_to_skip {
                    Skipsteps::NoJump => steps_to_skip = skipnextbestfield( k as usize, &list[index..list.len()-1] ),
                    Skipsteps::JumpOnce(_) | Skipsteps::JumpMany(_) => (),
                }
                if !steps_to_skip.is_during_jump() {
                    bmp.set(index)
                }
                steps_to_skip.decrease_one();
            },
            (_,_) => ()
        }
    });
    bmp.set(list.len()-1);
    bmp
}

fn skipnextbestfield(k: usize, list:&[i32]) -> Skipsteps {
    for i in 1..min(k as usize, list.len())  {
        if list[i] >= 0 {return Skipsteps::JumpOnce(i);}
    }
    if k as usize >= list.len() {
        return Skipsteps::JumpOnce(list.len());
    }
    for i in k..list.len(){
        if list[i] >= 0 || i == list.len() -1{
            let mut v:Vec<usize> = (0..((i+k)/k)).map(|f| ((f+1) * k)).collect();
            if i%k != 0 {
                v.push(i%k);
            }
            return Skipsteps::JumpMany(RefCell::new(v));
        }
    }
    //logic für strategie
    Skipsteps::NoJump
}

enum Skipsteps{
    NoJump,
    JumpOnce(usize),
    JumpMany(RefCell<Vec<usize>>)
}

impl Skipsteps {

    fn decrease_one(&mut self){
        match self {
            Skipsteps::NoJump => (),
            Skipsteps::JumpOnce(val) if *val <= 1 => *self = Skipsteps::NoJump,
            Skipsteps::JumpOnce(val) => *val -= 1,
            Skipsteps::JumpMany(list) if *list.borrow().get(0).unwrap() >= 1 => *list.borrow_mut().get_mut(0).unwrap() -= 1,
            Skipsteps::JumpMany(list) => {
                list.borrow_mut().remove(0);
                let len = list.borrow().len();
                /*match len { // Das muss ich noch verstehen, warum das hier nicht geht, die unere If bedingung aber ohne Probleme funktioniert
                    x if x == 0 => (* self) = {let f = list.to_owned(); Skipsteps::NoJump},
                    x if x == 1 => * self = Skipsteps::JumpOnce(*list.borrow().get(0).unwrap()),
                    _ => ()
                }*/
                if len == 1 {
                    *self = Skipsteps::JumpOnce(*list.to_owned().borrow().get(0).unwrap());
                }else if len == 0 {
                    *self = Skipsteps::NoJump;
                }
            }
        }
    }

    fn is_during_jump(&self) -> bool{
        match self {
            Skipsteps::NoJump => false, // falls das Springen nicht nötig ist
            Skipsteps::JumpOnce(_) => true, // wenn nur ein Element weiter gesprungen werden muss
            Skipsteps::JumpMany(list) if *list.borrow().get(0).unwrap_or(&0) >= 1 => true, // wenn bei mehreren Sprüngen der erste Sprung noch nicht erreicht wurde
            _ => false                      // wenn bei mehreren Sprüngen das ende des ersten erreicht wurde
        }
    }

    fn add_jump_to_beginning(self, jumplength:usize) -> Self {
        match self {
            Skipsteps::NoJump => Skipsteps::JumpOnce(jumplength),
            Skipsteps::JumpOnce(x) => Skipsteps::JumpMany(RefCell::new(vec![jumplength,x])),
            Skipsteps::JumpMany(v) => {v.borrow_mut().insert(0, jumplength); Skipsteps::JumpMany(v)},
        }
    }

}

struct Bitmap {
    data:Vec<u32>
}

impl Bitmap {
    pub fn new(size:usize) -> Bitmap{
        Bitmap {
            data: vec![0; (size>>5)+1]
        }
    }
    pub fn set(& mut self, index:usize){
        let number_index = (index) >> 5;
        let bit_index =  index & 0x1F;
        let bit_val = (0x80000000 as u32) >> bit_index;
        let num_val = self.data.get_mut(number_index).unwrap();
        *num_val = (*num_val) | bit_val;

    }
    pub fn print(&self){
        println!("{:?}", self.data);
    }
}
