use core::file;

use crate::{
    cstr::Cstr, nxopen_ui::uiblock::{self, UIBlock}, syss, taggedobject::{Tag, TaggedObject}, utilities::jam::jam_start_wrapped_call
};

pub struct CompositeBlock {
    pub(crate) tag: Tag,
    pub(crate) ptr: usize,
}

impl TaggedObject for CompositeBlock {
   fn get_tag(&self) -> crate::taggedobject::Tag {
        self.tag
    }

    fn get_ptr(&self) -> usize {
        self.ptr
    }
}
impl UIBlock for CompositeBlock {
    fn get_enable(&self) -> bool {
        let mut e: bool = false;
        jam_start_wrapped_call();
        let num = uiblock::xja_block_styler_get_enable(self.ptr, &mut e);
        if num != 0 {
            let err = syss::decode_error(num);
            syss::error_internal(Cstr::from(file!()).ptr, line!(), err);
        }
        e
    }

    fn set_enable(&self, value: bool) {
        uiblock::xja_block_styler_set_enable(self.ptr, value);
    }

    fn get_expanded(&self) -> bool {
        let mut e: bool = false;
        uiblock::xja_block_styler_get_expanded(self.ptr, &mut e);
        e
    }

    fn set_expanded(&self, value: bool) {
        uiblock::xja_block_styler_set_expanded(self.ptr, value);
    }

    fn get_group(&self) -> bool {
        let mut e: bool = false;
        uiblock::xja_block_styler_get_group(self.ptr, &mut e);
        e
    }

    fn set_group(&self, value: bool) {
        uiblock::xja_block_styler_set_group(self.ptr, value);
    }

    fn get_label(&self) -> *const u8 {
        let mut e: *mut u8 = 0 as _;
        uiblock::xja_block_styler_get_label(self.ptr, &mut e);
        e
    }

    fn set_label(&self, value: *const u8) {
        uiblock::xja_block_styler_set_label(self.ptr, value);
    }

    fn get_name(&self) -> *const u8 {
        let mut e: *mut u8 = 0 as _;
        uiblock::xja_block_styler_get_name(self.ptr, &mut e);
        e
    }

    fn get_show(&self) -> bool {
        let mut e: bool = false;
        uiblock::xja_block_styler_get_show(self.ptr, &mut e);
        e
    }

    fn set_show(&self, value: bool) {
        uiblock::xja_block_styler_set_show(self.ptr, value);
    }

    fn get_type(&self) -> *const u8 {
        let mut e: *mut u8 = 0 as _;
        uiblock::xja_block_styler_get_type(self.ptr, &mut e);
        e
    }

    fn focus(&self) {
        uiblock::xja_block_styler_test_focus_change(self.ptr);
    }
}
