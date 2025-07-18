use crate::{
    syss::error_internal,
    winapi::{WideCharToMultiByte, free, malloc},
};

trait Cstr {
    fn as_cstr(&self) -> *const u8;
    fn as_asni(&self) -> *const u8;
}

impl Cstr for &str {
    fn as_cstr(&self) -> *const u8 {
        let size = self.len() + 1;
        let ptr = unsafe { malloc(size) };
        if ptr.is_null() {
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping((*self).as_ptr(), ptr, self.len());
            *ptr.add(self.len()) = 0;
        }
        ptr
    }

    fn as_asni(&self) -> *const u8 {
        let mut code_points: Vec<u16> = (*self).encode_utf16().collect();
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
            return ptr;
        }
        code_points.push(0);

        let required_size = unsafe {
            WideCharToMultiByte(
                0,
                0,
                code_points.as_ptr() as *const u8,
                -1,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
            )
        };

        if required_size <= 0 {
            error_internal("Cstr\0".as_ptr(), line!(), "required_size <= 0 \0".as_ptr());
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
                0,
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
        p
    }
}

trait Tostr {
    fn as_str(&self) -> &str;
    fn free(&self);
}
impl Tostr for *const u8 {
    fn as_str(&self) -> &str {
        unsafe {
            match std::ffi::CStr::from_ptr(*self as _).to_str() {
                Ok(s) => s,
                Err(_) => {
                    error_internal(
                        "Cstr\0".as_ptr(),
                        line!(),
                        "CStr creation failed\0".as_ptr(),
                    );
                    "\0"
                }
            }
        }
    }
    fn free(&self) {
        unsafe {
            free((*self) as _);
        }
    }
}
