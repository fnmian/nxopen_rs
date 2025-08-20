use crate::Tag;


#[link(name = "libjam", kind = "dylib")]
unsafe extern "C" {
   #[link_name = "JAM_start_wrapped_call"]
    pub fn JAM_start_wrapped_call();
    pub fn JAM_lookup_singleton_tag(name: *const u8) -> Tag;
}

#[link(name = "libsyss", kind = "dylib")]
unsafe extern "C" {
   #[link_name = "?ERROR_decode@@YAPEADH@Z"]
    pub fn ERROR_decode(code:i32)->*const u8;
    #[link_name = "?ERROR_internal@@YAXPEBDH0ZZ"]
    pub fn ERROR_internal(classname: *const u8, error_line: u32, error_content: *const u8);
    #[link_name = "?UG_INSPECT_version@@YAPEBDXZ"]
    pub fn get_release()->*const u8;
}