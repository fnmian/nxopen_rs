use crate::{
    syss::error_internal,
    winapi::{WideCharToMultiByte, free, malloc},
};

///C风格字符串
#[repr(C)]
pub struct Cstr {
    data: *const u8,
}

impl Cstr {
    ///创建utf8编码字符串
    pub fn new(str: &str) -> Self {
        let size = str.len() + 1;
        let ptr = unsafe { malloc(size) };
        if ptr.is_null() {
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(str.as_ptr(), ptr, str.len());
            *ptr.add(str.len()) = 0;
        }
        Self { data: ptr }
    }
    ///创建asni编码字符串
    pub fn new_asni(str: &str) -> Self {
        let mut code_points: Vec<u16> = str.encode_utf16().collect();
        if code_points.is_empty() {
            let ptr = unsafe { malloc(1) };
            if ptr.is_null() {
                error_internal(
                    "Cstr\0".as_ptr(),
                    line!(),
                    "Failed to allocate memory\0".as_ptr(),
                );
            }
            unsafe {
                *ptr = 0;
            }
            return Self { data: ptr };
        }
        code_points.push(0);

        let required_size = unsafe {
            WideCharToMultiByte(
                0,
                1024,
                code_points.as_ptr() as *const u8,
                -1,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
            )
        };

        if required_size <= 0 {
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "required_size <= 0 \0".as_ptr(),
            );
        }

        let buffer_size = required_size as usize;
        let p = unsafe { malloc(buffer_size) };

        if p.is_null() {
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
        }
        let result = unsafe {
            WideCharToMultiByte(
                0,
                1024,
                code_points.as_ptr() as *const u8,
                -1,
                p,
                buffer_size as i32,
                std::ptr::null(),
                std::ptr::null_mut(),
            )
        };
        if result <= 0 {
            unsafe { free(p as _) };
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "Failed to WideCharToMultiByte\0".as_ptr(),
            );
        }
        Self { data: p }
    }
}
impl Drop for Cstr {
    fn drop(&mut self) {
        unsafe {
            free(self.data as _);
        }
    }
}
