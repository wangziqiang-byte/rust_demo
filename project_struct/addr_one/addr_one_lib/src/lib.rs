mod db;
use crate::db::db_close::close;
use crate::db::db_open::open;
use crate::db::db_switch::pub_switch;


pub fn add_one_lib_print() {
    println!("add_one_lib_print")
}

fn inner_db() {
    println!("inner_db");
    open();
    pub_switch();
    close();
}

pub fn pub_db() {
    println!("pub_db");
    open();
    pub_switch();
    close();
}