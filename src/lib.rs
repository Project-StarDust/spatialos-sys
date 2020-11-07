#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate gdi32;
extern crate user32;

pub fn generate_worker() -> () {
    let parameters = unsafe {
        Worker_DefaultConnectionParameters()
    };
    println!("{:?}", parameters);
}