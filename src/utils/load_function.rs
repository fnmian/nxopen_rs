
#[link(name = "kernel32", kind = "dylib")]
unsafe extern "C" {
    pub fn LoadLibraryA(dllname: *const u8) -> usize;
    pub fn GetProcAddress(dll: usize, fn_name: *const u8) -> usize;
}
unsafe extern "C" {
    pub fn strlen(cs: *const u8) -> usize;
}

#[macro_export]
macro_rules! func {
    (
        $(#[$doc:meta])*
        $vis:vis fn $fn_name:ident($($arg:ident: $arg_type:ty),*) $(-> $ret:ty)? {
            dll: $dll_name:literal,
            func: $func_name:literal
        }
    ) => {
        $(#[$doc])*
        $vis fn $fn_name($($arg: $arg_type),*) $(-> $ret)? {
            static mut FUNC_PTR: usize = 0;
            unsafe {
                if FUNC_PTR==0 {
                    let dll_name_cstr = concat!($dll_name, "\0");
                    let func_name_cstr = concat!($func_name, "\0");
                    let dll = LoadLibraryA(dll_name_cstr.as_ptr());
                    if dll==0 {
                       std::arch::asm!("int 3");
                    }
                    let proc = GetProcAddress(dll, func_name_cstr.as_ptr());
                    if proc==0 {
                        std::arch::asm!("int 3");
                    }
                    FUNC_PTR = proc as usize;
                }
                let func: extern "system" fn($($arg_type),*) $(-> $ret)? = 
                    std::mem::transmute(FUNC_PTR);
                func($($arg),*)
            }
        }
    };
}
