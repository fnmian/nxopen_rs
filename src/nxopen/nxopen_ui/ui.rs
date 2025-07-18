use crate::{
    lazy_load_function,taggedobject::{Tag, TaggedObject}
};

pub struct UI {
    tag: Tag,
    ptr: usize,
}

impl UI {}

impl TaggedObject for UI {
    fn get_tag(&self) -> Tag {
        self.tag
    }

    fn get_ptr(&self) -> usize {
        self.ptr
    }
}

lazy_load_function! {
    ///需要asni编码字符串
    pub fn uc1601(a:*const u8, b: i32){dll:"libugopenint.dll",func:"uc1601"}
}

pub fn print(s:*const u8){
    crate::syss::list_uiprintf(s);
}
