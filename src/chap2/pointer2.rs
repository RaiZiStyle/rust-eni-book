#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn stuff_pointer() {
    let integer: i32 = 8;
    println!("integer ref : {:p}", &integer.clone());
}
