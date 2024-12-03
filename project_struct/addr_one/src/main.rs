use addr_lib::add_lib_print;
use addr_one_lib::{add_one_lib_print, pub_db};

fn main() {
    add_one_lib_print();
    pub_db();
    add_lib_print();
}

pub fn addr_one_method() {
    println!("addr_one_method")
}