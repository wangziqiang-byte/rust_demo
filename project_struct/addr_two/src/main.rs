use addr_one_lib::pub_db;

fn main() {
    println!("Hello, world!2");
    pub_two();
}

fn pub_two() {
    println!("two");
    // addr_one -> addr_one_lib
    pub_db();
}