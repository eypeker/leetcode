//use wasm_bindgen::prelude::*;

/*#[wasm_bindgen]
extern {
    pub fn alert(s:&str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("Helloe, {}!", name));
}*/


mod math {
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


//#[export_name = "wasm_add_two_numbers"]
/*#[no_mangle]
pub fn add(left:f64, right:f64) -> f64 {
    left + right + unsafe {random()}
}*/


#[export_name = "add"]
pub extern "C" fn add( left:f64, right: f64) -> f64 {
    left + right + math::random()
}

