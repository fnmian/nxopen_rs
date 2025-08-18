
#[link(name = "libsyss", kind = "dylib")]
unsafe extern "C" {
   #[link_name = "?listUIprintf@@YAXPEBDZZ"]
    pub fn list_uiprintf(format: *const u8, ...);
}

#[macro_export]
macro_rules! nx_println {
($($arg:tt)*) => {{
    let mut formatted_string = format!($($arg)*);
    formatted_string.push('\n');
    let c_string = nxopen_rs::cstr::Cstr::from(formatted_string.as_str());
    if !c_string.ptr.is_null() {
    unsafe {
       nxopen_rs::list_ui::list_uiprintf(c_string.ptr);
        }
    }}};
}