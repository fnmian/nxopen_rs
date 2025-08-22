use core::fmt;

use crate::{
    jam::{SM_alloc, SM_free},
     p_s,
    winapi::{WideCharToMultiByte, memcpy, strlen},
};

pub struct Cstr {
    pub(crate) ptr: *const u8,
}
impl Cstr {
    #[inline]
    pub fn empty() -> Self {
        let mut ptr = unsafe { SM_alloc(1) };
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
    pub fn from_ptr(src: *const u8, do_free: bool) -> Self {
        let len = unsafe { strlen(src) + 1 };
        let mut ptr = unsafe { SM_alloc(len) };
        if ptr.is_null() {
            ptr = 0 as _;
        } else {
            unsafe { memcpy(ptr as _, src as _, len) };
        }
        if do_free {
            unsafe { SM_free(src) };
        }
        Cstr { ptr: ptr as _ }
    }
    #[inline]
    pub const fn get_ptr(&self) -> *const u8 {
        self.ptr
    }
}
impl Drop for Cstr {
    #[inline]
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { SM_free(self.ptr) };
            self.ptr = 0 as _;
        }
    }
}

impl From<&str> for Cstr {
    #[inline]
    fn from(value: &str) -> Self {
        let mut ptr = unsafe { SM_alloc(value.len() + 1) };
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
        let mut ptr = unsafe { SM_alloc(len + 1) };
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
impl fmt::Display for Cstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ptr.is_null() {
            write!(f, "String.empty")
        } else {
            write!(f, "{}", p_s!(self.ptr))
        }
    }
}

unsafe impl Send for Cstr {}
unsafe impl Sync for Cstr {}

pub struct CstrAnsi {
    pub(crate) ptr: *const u8,
}
impl CstrAnsi {
    #[inline]
    pub fn new(s: &str) -> CstrAnsi {
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
            unsafe { std::arch::asm!("int 3") };
        }

        let buffer_size = required_size as usize;
        let p = unsafe { SM_alloc(buffer_size) };

        if p.is_null() {
            unsafe { std::arch::asm!("int 3") };
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
            unsafe { SM_free(p as _) };
            unsafe { std::arch::asm!("int 3") };
        }
        CstrAnsi { ptr: p }
    }
    #[inline]
    pub const fn get_ptr(&self) -> *const u8 {
        self.ptr
    }
}
impl Drop for CstrAnsi {
    #[inline]
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { SM_free(self.ptr) };
        }
    }
}
