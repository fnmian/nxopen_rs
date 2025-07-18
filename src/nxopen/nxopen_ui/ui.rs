use crate::{lazy_load_function, taggedobject::Tag};

pub struct UI {
    tag: Tag,
    ptr: usize,
}

impl UI {
    lazy_load_function! {
        ///需要asni编码字符串
        pub fn uc1601(a:*const u8, b: i32){dll:"libugopenint.dll",func:"uc1601"}
    }
}
