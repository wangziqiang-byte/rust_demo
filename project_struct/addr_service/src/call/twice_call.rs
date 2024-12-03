use crate::call::call_import::important_source::important_source;
use crate::call::twice_call::call_crate::call::call_crate;
use crate::call::twice_call::call_pub::call::call_pub;
// is private 
// use crate::call::twice_call::call_parent_path::call_path::call::call_path;
/**
is private -> 如果需要解决此调用问题可以将call_self.rs中
use crate::call::twice_call::call_self::call::call_self;
替换为
pub use crate::call::twice_call::call_self::call::call_self;
重新导出即可
*/
// use crate::call::twice_call::call_self::call_self;
use crate::call::twice_call::call_super::call::call_super;

pub mod call_crate;
pub mod call_pub;
pub mod call_self;
pub mod call_super;
mod call_parent_path;

fn twice_call() {
    important_source();
    call_crate();
    call_super();
    call_pub();
    // 仅限于call_self.rs中
    // call_self();
    // 仅限于crate::call::twice_call::call_parent_path及一下的包调用
    // call_path();
}
