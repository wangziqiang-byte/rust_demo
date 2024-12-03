mod call;
mod error;
// is private
// use crate::call::call_import::important_source::important_source;
use addr_lib::*;
//error is private -> pub(self) 在当前模块可见
// use crate::error::e_auth::auth;
use crate::error::e_check::check;

use crate::call::twice_call::call_crate::call::call_crate;
use crate::call::twice_call::call_pub::call::call_pub;

fn main() {
    add_lib_print();
    pub_debug("从addr_lib 中 pub use 重导出的");
    // error : cannot find this function
    // pub_log("从addr_lib 中 pub use 重导出的");
    check();
    // auth();
    // error
    // important_source();
    call_crate();
    call_pub();
}