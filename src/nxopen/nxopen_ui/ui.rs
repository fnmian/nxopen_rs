use crate::{
    cstr::Cstr, lazy_load_function, nxopen_ui::block_dialog::BlockDialog, syss, taggedobject::{Tag, TaggedObject}, utilities::jam
};


pub struct UI {
    tag: Tag,
    ptr: usize,
}

impl UI {
#[allow(non_snake_case)]
    pub fn GetUI() -> Self {
        let t =jam::lookup_singleton_tag("NXOpen.UI\0".as_ptr());
        let ptr = jam::jam_lookup_tag(t);
        Self { tag: t, ptr: ptr }
    }
    pub fn create_dialog(dialog_name: &Cstr) -> Result<BlockDialog, *const u8> {
        let mut dialog: usize = 0;
        jam::jam_start_wrapped_call();
        let errcode = ui_main_create_dialog(dialog_name.ptr, &mut dialog);
        if errcode != 0 {
            let err = syss::decode_error(errcode);
            return Err(err);
        }
        Ok(BlockDialog { ptr: dialog })
    }
}

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
lazy_load_function! {
    pub fn ui_main_create_dialog(dialog_name: *const u8, dialog: &mut usize) -> i32{
        dll: "libgeomint.dll",
        func: "XJA_UI_MAIN_create_dialog"
    }
}

pub fn print(s:&Cstr){
    crate::syss::list_uiprintf(s.ptr);
}
