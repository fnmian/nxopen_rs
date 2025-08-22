use crate::Tag;


#[link(name = "libjam", kind = "dylib")]
unsafe extern "C" {
   #[link_name = "JAM_start_wrapped_call"]
    pub fn JAM_start_wrapped_call();
    pub fn JAM_lookup_singleton_tag(name: *const u8) -> Tag;
    pub fn JAM_ask_object_tag(obj:usize)->Tag;
    #[link_name = "JAM_sm_alloc"]
    pub fn SM_alloc(nbytes:usize)->*mut u8;
     #[link_name = "JAM_sm_free"]
    pub fn SM_free(ptr:*const u8);
      #[link_name = "JAM_text_free"]
    pub fn TEXT_free(ptr:*const u8);
      #[link_name = "JAM_text_create_string"]
    pub fn TEXT_create_string(str:*const u8)->usize;
}

#[link(name = "libsyss", kind = "dylib")]
unsafe extern "C" {
   #[link_name = "?ERROR_decode@@YAPEADH@Z"]
    pub fn ERROR_decode(code:i32)->*const u8;
    #[link_name = "?ERROR_internal@@YAXPEBDH0ZZ"]
    pub fn ERROR_internal(classname: *const u8, error_line: u32, error_content: *const u8);
    #[link_name = "?UG_INSPECT_version@@YAPEBDXZ"]
    pub fn get_release()->*const u8;
    #[link_name = "?TAG_ask_pointer_of_tag@@YAPEAXI@Z"]
    pub fn TAG_ask_pointer_of_tag(tag:Tag)->usize;
     #[link_name = "?TEXT_to_locale@@YAPEADPEBUTEXT_s@@@Z"]
    pub fn TEXT_to_locale(text:usize)->*const u8;
}