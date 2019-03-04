#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/xen-bindings.rs"));

pub fn init() {
    unsafe {
        println!("hello XS ! {:p}", xs_open(XBT_NULL.into()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        init()
    }
}
