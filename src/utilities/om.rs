use crate::{Tag};



#[link(name = "libsyss", kind = "dylib")]
unsafe extern "C" {
    #[link_name = "?TAG_ask_pointer_of_tag@@YAPEAXI@Z"]
    pub fn TAG_ask_pointer_of_tag(tag:Tag)->usize;
}