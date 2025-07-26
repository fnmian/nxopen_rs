use crate::{
    syss::error_internal,
    winapi::{WideCharToMultiByte, free, malloc, strlen},
};

pub struct Cstr {
    pub  ptr: *mut u8,
}
impl Cstr {
    #[inline]
    pub fn empty() -> Self {
        let mut ptr = unsafe { malloc(1) };
        if ptr.is_null() {
            ptr = 0 as _;
        } else {
            unsafe {
                *ptr = 0;
            }
        }
        Cstr { ptr: ptr as _ }
    }
    #[inline]
   pub fn str_len(&self) -> usize {
        unsafe { strlen(self.ptr) }
    }
    #[inline]
    pub  fn new_ansi(s:&str) -> Cstr {
        let mut code_points: Vec<u16> = s.encode_utf16().collect();
        code_points.push(0);

        let required_size = unsafe {
            WideCharToMultiByte(
                0,
                0,
                code_points.as_ptr() as _,
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
                code_points.as_ptr() as _,
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
        Cstr { ptr: p }
    }
}
impl Drop for Cstr {
    #[inline]
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { free(self.ptr) };
        }
    }

}

impl From<&str> for Cstr {
    #[inline]
    fn from(value: &str) -> Self {
        let mut ptr = unsafe { malloc(value.len() + 1) };
        if ptr.is_null() {
            ptr = 0 as _;
        } else {
            unsafe {
                std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
                *ptr.add(value.len()) = 0;
            }
        }
        Cstr { ptr: ptr as _ }
    }
}
impl From<Cstr> for String {
    #[inline]
    fn from(value: Cstr) -> Self {
       unsafe {
            std::ffi::CStr::from_ptr(value.ptr as _)
                .to_string_lossy()
                .into_owned()
        }
    }
}
impl Clone for Cstr {
    fn clone(&self) -> Self {
        if self.ptr.is_null() {
            return Cstr::empty();
        }
        let len = self.str_len();
        let mut ptr = unsafe { malloc(len + 1) };
        if ptr.is_null() {
            ptr = 0 as _;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr, ptr, len + 1);
        }
        Cstr { ptr }
    }
}

impl Default for Cstr {
    fn default() -> Self {
        Cstr::empty()
    }
}

unsafe impl Send for Cstr {}
unsafe impl Sync for Cstr {}
