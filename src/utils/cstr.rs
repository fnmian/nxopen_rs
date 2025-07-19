use std::ops::{Add, AddAssign};

use crate::{
    syss::error_internal,
    winapi::{WideCharToMultiByte, free, malloc, realloc, strlen},
};

pub struct Cstr {
    pub ptr: *mut u8,
}
impl Cstr {
    pub fn empty() -> Self {
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
        Self { ptr: ptr }
    }
    fn len(&self) -> usize {
        unsafe { strlen(self.ptr) }
    }
    pub fn push(&mut self, c: u8) {
        let current_len = self.len();
        let new_len = current_len + 1;
        let new_size = new_len + 1;

        let new_ptr = unsafe { realloc(self.ptr as *mut _, new_size) };
        if new_ptr.is_null() {
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "Failed to reallocate memory\0".as_ptr(),
            );
        }
        self.ptr = new_ptr;

        unsafe {
            let dest = self.ptr.offset(current_len as isize);
            *dest = c;
            *dest.offset(1) = 0;
        }
    }
    pub fn pop(&mut self) -> Option<u8> {
        let current_len = self.len();
        if current_len == 0 {
            return None;
        }
        let c = unsafe { *self.ptr.offset((current_len - 1) as isize) };
        unsafe {
            *self.ptr.offset((current_len - 1) as isize) = 0;
        }
        Some(c)
    }
    pub fn append(&mut self, other: &Cstr) {
        let other_len = other.len();
        if other_len == 0 {
            return;
        }
        let current_len = self.len();
        let new_len = current_len + other_len;
        let new_size = new_len + 1;

        let new_ptr = unsafe {realloc(self.ptr as *mut _, new_size) };
        if new_ptr.is_null() {
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "Failed to reallocate memory\0".as_ptr(),
            );
        }
        self.ptr = new_ptr as *mut u8;

        unsafe {
            let dest_start = self.ptr.offset(current_len as isize);
            let src_start = other.ptr;
            for i in 0..other_len as isize {
                *dest_start.offset(i) = *src_start.offset(i);
            }
            *dest_start.offset(other_len as isize) = 0;
        }
    }

}
impl Add<&Cstr> for &Cstr {
    type Output = Cstr;

    fn add(self, other: &Cstr) -> Cstr {
        let new_len = self.len() + other.len();
        let new_size = new_len + 1;

        let new_ptr = unsafe {malloc(new_size) };
        if new_ptr.is_null() {
            error_internal(
                "Cstr\0".as_ptr(),
                line!(),
                "Failed to allocate memory for '+' operator\0".as_ptr(),
            );
        }

        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len());
            std::ptr::copy_nonoverlapping(
                other.ptr,
                new_ptr.offset(self.len() as isize),
                other.len(),
            );
            *new_ptr.offset(new_len as isize) = 0;
        }

        Cstr { ptr: new_ptr as *mut u8 }
    }
}

impl AddAssign<&Cstr> for Cstr {
    fn add_assign(&mut self, other: &Cstr) {
        self.append(other);
    }
}
impl Drop for Cstr {
    #[inline]
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { free(self.ptr as _) };
        }
    }
}
pub trait CstrTrait {
    fn to_cstring(&self) -> Cstr;
    fn to_ansi(&self) -> Cstr;
}

impl CstrTrait for &str {
    fn to_cstring(&self) -> Cstr {
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
        Cstr { ptr: ptr }
    }

    fn to_ansi(&self) -> Cstr {
        let mut code_points: Vec<u16> = self.encode_utf16().collect();
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

pub trait Tostr {
    fn to_string(self) -> String;
}
impl Tostr for Cstr {
    fn to_string(self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(self.ptr as _)
                .to_string_lossy()
                .into_owned()
        }
    }
}
