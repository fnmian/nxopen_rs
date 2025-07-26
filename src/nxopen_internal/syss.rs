use crate::{
    lazy_load_function,
    winapi::{GetProcAddress, LoadLibraryA},
};

static mut FN_ERROR_INTERNAL: usize = 0;

///nx异常函数，可中断执行
#[inline(never)]
pub fn error_internal(classname: *const u8, error_line: u32, error_content: *const u8) {
    unsafe {
        if FN_ERROR_INTERNAL == 0 {
            FN_ERROR_INTERNAL = GetProcAddress(
                LoadLibraryA("libsyss.dll\0".as_ptr()),
                "?ERROR_internal@@YAXPEBDH0ZZ\0".as_ptr(),
            );
        }
        (*((&raw const FN_ERROR_INTERNAL) as *const _ as *const fn(*const u8, u32, *const u8)))(
            classname,
            error_line,
            error_content,
        );
    }
}

lazy_load_function! {
    pub fn get_release()->*const u8{dll:"libsyss.dll",func:"?UG_INSPECT_version@@YAPEBDXZ"}
}
lazy_load_function! {
    pub fn decode_error(code:i32) -> *const u8{dll:"libsyss.dll",func:"?ERROR_decode@@YAPEADH@Z"}
}

#[link(name = "./libs/libsyss", kind = "dylib")]
unsafe extern "C" {
   #[link_name = "?listUIprintf@@YAXPEBDZZ"]
    pub fn list_uiprintf(format: *const u8, ...);
}
