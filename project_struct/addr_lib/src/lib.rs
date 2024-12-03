use addr_one_lib::pub_db;
pub use crate::log::log_debug::pub_debug;
use crate::log::log_info::pub_info;
// use crate::log::log_write::*; // error: module log_write is private
use crate::log::pub_log;

mod log;


pub fn add_lib_print() {
    // error，因为不是pub
    // log("");
    pub_log("pub_log");
    // error，因为不是pub
    // info("info");
    pub_info("info");
    // error，因为不是pub
    // debug("info");
    pub_debug("debug");
    
    println!("add_lib_print root");
    // 调用addr_one下addr_one_lib库的pub_db()
    pub_db();
}


