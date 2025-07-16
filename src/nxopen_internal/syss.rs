use crate::winapi::{GetProcAddress, LoadLibraryA};

static mut FN_ERROR_INTERNAL: usize = 0;

///nx异常函数，可中断执行
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
