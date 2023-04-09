use wasm_bindgen::prelude::*;



/*#[wasm_bindgen]
extern {
    pub fn alert(s:&str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("Helloe, {}!", name));
}*/


//#[no_std]
/*#[wasm_bindgen]
pub fn add( left: i32, right: i32) ->i32{
    left+right
}
*/


pub fn trap(elevation: Vec<u32>) -> u32{
    sum_list(water_levels(elevation))
}


#[wasm_bindgen]
pub fn water_levels(elevation: Vec<u32>) -> Vec<u32>{
    let mut highest: u32 = 0;
    let mut waterlevel: Vec<u32> = vec![0; elevation.len()];
    for i in 0..elevation.len() {
        let f = match elevation[i] {
            k if k < highest => highest - k,
            k if k >= highest => {highest = k; 0},
            _ => 0
        };
        waterlevel[i] = f;
    }
    highest = 0;
    for i in (0..elevation.len()).rev() {
        let f = match elevation[i] {
            k if k < highest => highest - k,
            k if k >= highest => {highest = k; 0},
            _ => 0
        };
        waterlevel[i] = match waterlevel[i] {
            k if k < f => k,
            k if k >= f => f,
            _ => 0
        }
    }
    waterlevel
}

#[wasm_bindgen]
pub fn sum_list(list: Vec<u32>) -> u32{
    list.into_iter().sum()
}

/*mod math {
    mod math_js {
        #[link(wasm_import_module = "Math")]
        extern "C"{
            pub fn random() -> f64;
        }
    }

    pub fn random() -> f64 {
        unsafe {math_js::random()}
    }
}
*/

//#[export_name = "wasm_add_two_numbers"]
//#[no_mangle]
/*pub fn add(left:f64, right:f64) -> f64 {
    left + right + unsafe {random()}
}*/

/*
#[export_name = "add"]
pub extern "C" fn add( left:f64, right: f64) -> f64 {
    left + right + math::random()
}

*/




#[test]
pub fn onlinebeispiel(){
    let feld = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let erg = trap(feld);
    assert_eq!(erg,6);
}


#[test]
pub fn mitlinkerand(){
    let feld = vec![2,1,0,2,1,0,1,3,2,1,2,1];
    let erg = trap(feld);
    assert_eq!(erg,8);
}

#[test]
pub fn mitrechterand(){
    let feld = vec![0,1,0,2,1,0,1,3,2,1,2,3];
    let erg = trap(feld);
    assert_eq!(erg,9);
}

#[test]
pub fn mitlinkerechterand(){
    let feld = vec![2,1,0,2,1,0,1,3,2,1,2,3];
    let erg = trap(feld);
    assert_eq!(erg,11);
}