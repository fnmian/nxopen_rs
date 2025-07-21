use crate::{lazy_load_function};


lazy_load_function! {
    pub fn lookup_singleton_tag(name: *const u8) -> u32 {
        dll: "libjam.dll",
        func: "JAM_lookup_singleton_tag"
    }
}
lazy_load_function! {
    pub fn jam_lookup_tag(tag:u32)->usize{
        dll: "libjam.dll",
        func: "JAM_lookup_tag"
    }
}
lazy_load_function! {
    pub fn jam_ask_object_tag(ptr:usize)->u32{
        dll: "libjam.dll",
        func: "JAM_ask_object_tag"
    }
}
lazy_load_function! {
    pub fn jam_start_wrapped_call(){
        dll: "libjam.dll",
        func: "JAM_start_wrapped_call"
    }
}
lazy_load_function! {
    pub fn jam_decode_error(errcode:i32, errbuf:&mut u8){
        dll: "libjam.dll",
        func: "JAM_decode_error"
    }
}