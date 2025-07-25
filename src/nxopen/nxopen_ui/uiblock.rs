use crate::{lazy_load_function, taggedobject::TaggedObject};

pub trait UIBlock: TaggedObject {
    fn get_enable(&self) -> bool;
    fn set_enable(&self, value: bool);
    fn get_expanded(&self) -> bool;
    fn set_expanded(&self, value: bool);
    fn get_group(&self) -> bool;
    fn set_group(&self, value: bool);
    fn get_label(&self) -> *const u8;
    fn set_label(&self, value: *const u8);
    fn get_name(&self) -> *const u8;
    fn get_show(&self) -> bool;
    fn set_show(&self, value: bool);
    fn get_type(&self) -> *const u8;
    fn focus(&self);
}


lazy_load_function! {
    pub fn xja_block_styler_get_enable(block:usize, enable: &mut bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_get_enable"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_set_enable(block:usize, enable:bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_set_enable"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_get_expanded(block:usize, expanded: &mut bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_get_expanded"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_set_expanded(block:usize, expanded:bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_set_expanded"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_get_group(block:usize, group: &mut bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_get_group"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_set_group(block:usize, group:bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_set_group"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_get_label(block:usize, enable: &*mut u8)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_get_label"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_set_label(block:usize, enable:*const u8)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_set_label"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_get_name(block:usize, name:&*mut u8)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_get_name"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_get_show(block:usize, show: &mut bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_get_show"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_set_show(block:usize, show:bool)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_set_show"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_get_type(block:usize, block_type:&*mut u8)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_get_type"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_test_focus_change(block:usize)->i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_test_focus_change"
    }
}