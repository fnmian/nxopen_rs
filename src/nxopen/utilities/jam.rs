use crate::lazy_load_function;
pub struct JAM;
impl JAM {
    lazy_load_function! {
        pub fn lookup_singleton_tag(name: *const u8) -> u32 {
            dll: "libjam.dll",
            func: "JAM_lookup_singleton_tag"
        }
    }
}
