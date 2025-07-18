mod nxopen_ui;

pub mod utilities;
pub use nxopen_ui::*;
pub mod taggedobject;

#[macro_export]
macro_rules! lazy_load_function {
    (
        $(#[$doc:meta])*
        $vis:vis fn $fn_name:ident($($arg:ident: $arg_type:ty),*) -> $ret:ty {
            dll: $dll_name:literal,
            func: $func_name:literal
        }
    ) => {
        $(#[$doc])*
        $vis fn $fn_name($($arg: $arg_type),*) -> $ret {
            static mut FUNC_PTR: usize = 0;
            static mut DLL_HANDLE: usize = 0;

            unsafe {
                if FUNC_PTR == 0 {
                    let dll_str = concat!($dll_name, "\0");
                    let func_str = concat!($func_name, "\0");
                    let dll = crate::winapi::LoadLibraryA(dll_str.as_ptr());
                    if dll==0 {
                        crate::syss::error_internal(
                            dll_str.as_ptr(),
                            line!(),
                            concat!("Failed to load DLL: ", $dll_name, "\0").as_ptr()
                        );
                    }
                    let proc = crate::winapi::GetProcAddress(dll, func_str.as_ptr());
                    if proc==0 {
                        crate::syss::error_internal(
                            dll_str.as_ptr(),
                            line!(),
                            concat!("Failed to find function: ", $func_name, "\0").as_ptr()
                        );
                    }
                    DLL_HANDLE = dll as usize;
                    FUNC_PTR = proc as usize;
                }
                let func: extern "system" fn($($arg_type),*) -> $ret =
                    std::mem::transmute(FUNC_PTR);
                func($($arg),*)
            }
        }
    };

    (
        $(#[$doc:meta])*
        $vis:vis fn $fn_name:ident($($arg:ident: $arg_type:ty),*) {
            dll: $dll_name:literal,
            func: $func_name:literal
        }
    ) => {
        $(#[$doc])*
        $vis fn $fn_name($($arg: $arg_type),*) {
            static mut FUNC_PTR: usize = 0;
            static mut DLL_HANDLE: usize = 0;

            unsafe {
                if FUNC_PTR == 0 {
                    let dll_str = concat!($dll_name, "\0");
                    let func_str = concat!($func_name, "\0");
                    let dll = crate::winapi::LoadLibraryA(dll_str.as_ptr());
                    if dll==0 {
                        crate::syss::error_internal(
                            dll_str.as_ptr(),
                            line!(),
                            concat!("Failed to load DLL: ", $dll_name, "\0").as_ptr()
                        );
                    }
                    let proc = crate::winapi::GetProcAddress(dll, func_str.as_ptr());
                    if proc==0 {
                        crate::syss::error_internal(
                            dll_str.as_ptr(),
                            line!(),
                            concat!("Failed to find function: ", $func_name, "\0").as_ptr()
                        );
                    }
                    DLL_HANDLE = dll as usize;
                    FUNC_PTR = proc as usize;
                }
                let func: extern "system" fn($($arg_type),*) -> () =
                    std::mem::transmute(FUNC_PTR);
                func($($arg),*);
            }
        }
    };
}
