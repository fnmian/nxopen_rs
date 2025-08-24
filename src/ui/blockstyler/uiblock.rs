use crate::{
    blockstyler::{button::Button, selectobject::SelectObject}, cstr::NXstr, jam::{
        self, JAM_ask_object_tag, JAM_start_wrapped_call, TEXT_create_string, TEXT_free,
        TEXT_to_locale,
    }, taggedobject::TaggedObject, Tag
};

pub trait IUIBlock {
    fn block(&self)->UIBlock;
}

#[derive(Clone)]
pub struct UIBlock {
    pub(crate) ptr: usize,
}
impl PartialEq<Button> for UIBlock {
    fn eq(&self, other: &Button) -> bool {
        self.ptr==other.block().ptr
    }
}
impl PartialEq<SelectObject> for UIBlock {
    fn eq(&self, other: &SelectObject) -> bool {
        self.ptr==other.block().ptr
    }
}


impl UIBlock {
    pub fn new(p: usize) -> Self {
        UIBlock { ptr: p }
    }
    pub fn get_enable(&self) -> Result<bool, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut p = false;
        let n = unsafe { XJA_BLOCK_STYLER_get_enable(self.ptr, &mut p) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(p)
    }

    pub fn set_enable(&self, value: bool) -> Result<(), NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let n = unsafe { XJA_BLOCK_STYLER_set_enable(self.ptr, value) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(())
    }

    pub fn get_expanded(&self) -> Result<bool, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut p = false;
        let n = unsafe { XJA_BLOCK_STYLER_get_expanded(self.ptr, &mut p) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(p)
    }

    pub fn set_expanded(&self, value: bool) -> Result<(), NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let n = unsafe { XJA_BLOCK_STYLER_set_expanded(self.ptr, value) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(())
    }

    pub fn get_group(&self) -> Result<bool, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut p = false;
        let n = unsafe { XJA_BLOCK_STYLER_get_group(self.ptr, &mut p) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(p)
    }

    pub fn set_group(&self, value: bool) -> Result<(), NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let n = unsafe { XJA_BLOCK_STYLER_set_group(self.ptr, value) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(())
    }

    pub fn get_label(&self) -> Result<NXstr, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut p = 0;
        let n = unsafe { XJA_BLOCK_STYLER_get_label(self.ptr, &mut p) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        let str = unsafe { TEXT_to_locale(p) };
        unsafe { TEXT_free(p as _) };
        Ok(NXstr::from_nxstr_ptr(str, true))
    }

    pub fn set_label(&self, value: NXstr) -> Result<(), NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let text = unsafe { TEXT_create_string(value.ptr) };
        let n = unsafe { XJA_BLOCK_STYLER_set_label(self.ptr, text as _) };
        unsafe { TEXT_free(text as _) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(())
    }

    pub fn get_name(&self) -> Result<NXstr, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut p = 0;
        let n = unsafe { XJA_BLOCK_STYLER_get_name(self.ptr, &mut p) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(NXstr::from_nxstr_ptr(p as _, true))
    }

    pub fn get_show(&self) -> Result<bool, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut p = false;
        let n = unsafe { XJA_BLOCK_STYLER_get_show(self.ptr, &mut p) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(p)
    }

    pub fn set_show(&self, value: bool) -> Result<(), NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let n = unsafe { XJA_BLOCK_STYLER_set_show(self.ptr, value) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(())
    }

    pub fn get_type(&self) -> Result<NXstr, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut p = 0usize;
        let n = unsafe { XJA_BLOCK_STYLER_get_type(self.ptr, &mut p) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(NXstr::from_nxstr_ptr(p as _, true))
    }

    pub fn focus(&self) -> Result<(), NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let n = unsafe { XJA_BLOCK_STYLER_test_focus_change(self.ptr) };
        if n != 0 {
            return Err(NXstr::from_nxstr_ptr(unsafe { jam::ERROR_decode(n) }, false));
        }
        Ok(())
    }
}
impl TaggedObject for UIBlock {
    fn get_tag(&self) -> Tag {
        unsafe { JAM_ask_object_tag(self.ptr) }
    }

    fn get_ptr(&self) -> usize {
        self.ptr
    }
}
#[link(name = "libnxblockstyler", kind = "dylib")]
unsafe extern "C" {
    pub fn XJA_BLOCK_STYLER_get_enable(block: usize, enable: &mut bool) -> i32;
    pub fn XJA_BLOCK_STYLER_set_enable(block: usize, enable: bool) -> i32;
    pub fn XJA_BLOCK_STYLER_get_expanded(block: usize, expanded: &mut bool) -> i32;
    pub fn XJA_BLOCK_STYLER_set_expanded(block: usize, expanded: bool) -> i32;
    pub fn XJA_BLOCK_STYLER_get_group(block: usize, group: &mut bool) -> i32;
    pub fn XJA_BLOCK_STYLER_set_group(block: usize, group: bool) -> i32;
    pub fn XJA_BLOCK_STYLER_get_label(block: usize, enable: &mut usize) -> i32;
    pub fn XJA_BLOCK_STYLER_set_label(block: usize, enable: *const u8) -> i32;
    pub fn XJA_BLOCK_STYLER_get_name(block: usize, name: &mut usize) -> i32;
    pub fn XJA_BLOCK_STYLER_get_show(block: usize, show: &mut bool) -> i32;
    pub fn XJA_BLOCK_STYLER_set_show(block: usize, show: bool) -> i32;
    pub fn XJA_BLOCK_STYLER_get_type(block: usize, block_type: &mut usize) -> i32;
    pub fn XJA_BLOCK_STYLER_test_focus_change(block: usize) -> i32;
}