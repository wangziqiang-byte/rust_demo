pub mod log_info;
pub mod log_debug;
mod log_write;

fn log(log: &str) {
    println!("log inner log-> {}", log)
}

pub fn pub_log(log: &str) {
    println!("log inner pub_log -> {}", log)
}