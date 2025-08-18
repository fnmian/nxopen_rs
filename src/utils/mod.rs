pub mod load_function;
pub mod winapi;
pub mod cstr;

///&str=>const char*+\0
#[macro_export]
macro_rules! s {
    ($s:expr) => {{ concat!($s, "\0").as_ptr() }};
}
///&str=>const char*+\n\0
#[macro_export]
macro_rules! s_ln {
    ($s:expr) => {{ concat!($s, "\n\0").as_ptr() }};
}

///ptr=>String
#[macro_export]
macro_rules! p_ss {
    ($ptr:expr) => {
        unsafe{String::from_utf8_lossy(std::slice::from_raw_parts($ptr, strlen($ptr))).into_owned()}
    };
}

///ptr=>&str
#[macro_export]
macro_rules! p_s {
    ($ptr:expr) => {
       unsafe{std::str::from_utf8_unchecked(std::slice::from_raw_parts ($ptr, strlen($ptr)))}
    };
}

pub fn debugbreak(){
   unsafe {std::arch::asm!("int 3")};
}