mod bindgen {
    #![allow(clippy::all)]
    #![allow(unknown_lints)]
    #![allow(deref_nullptr)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings/bindings.rs"));
}

pub mod filament;
pub mod backend;
pub mod utils;
pub mod math;
pub mod filameshio;
pub mod ibl;
pub mod image;
