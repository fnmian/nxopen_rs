use crate::{
    cstr::Cstr,
    lazy_load_function,
    nxopen_ui::block_dialog::BlockDialog,
    syss,
    taggedobject::{Tag, TaggedObject},
    utilities::jam,
};

pub struct UI {
    tag: Tag,
    ptr: usize,
}

impl UI {
    #[allow(non_snake_case)]
    pub fn GetUI() -> Self {
        let t = jam::lookup_singleton_tag("NXOpen.UI\0".as_ptr());
        let ptr = jam::jam_lookup_tag(t);
        Self { tag: t, ptr: ptr }
    }
    pub fn create_dialog(dialog_name: &Cstr) -> Result<BlockDialog, i32> {
        let mut dialog: usize = 0;
        jam::jam_start_wrapped_call();
        let errcode = ui_main_create_dialog(dialog_name.ptr, &mut dialog);
        if errcode != 0 {
            return Err(errcode);
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

lazy_load_function! {
    pub fn list_uiprintf(format: *const u8){dll:"libsyss.dll",func:"?listUIprintf@@YAXPEBDZZ"}
}

#[macro_export]
macro_rules! nx_println {
($($arg:tt)*) => {{
    let mut formatted_string = format!($($arg)*);
    formatted_string.push('\n');
    let c_string = crate::Cstr::from(formatted_string.as_str());
    if !c_string.ptr.is_null() {
    unsafe {
            crate::ui::list_uiprintf(c_string.ptr);
        }
    }}};
}
