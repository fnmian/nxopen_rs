
    use crate::{CstrPtr, cstr::Cstr, itaggedobject::ITaggedObject};
    pub trait IUIBlock: ITaggedObject {
        fn get_enable(&self) -> Result<bool, CstrPtr>;
        fn set_enable(&self, value: bool) -> Result<(), CstrPtr>;
        fn get_expanded(&self) -> Result<bool, CstrPtr>;
        fn set_expanded(&self, value: bool) -> Result<(), CstrPtr>;
        fn get_group(&self) -> Result<bool, CstrPtr>;
        fn set_group(&self, value: bool) -> Result<(), CstrPtr>;
        fn get_label(&self) -> Result<Cstr, CstrPtr>;
        fn set_label(&self, value: Cstr) -> Result<(), CstrPtr>;
        fn get_name(&self) -> Result<Cstr, CstrPtr>;
        fn get_show(&self) -> Result<bool, CstrPtr>;
        fn set_show(&self, value: bool) -> Result<(), CstrPtr>;
        fn get_type(&self) -> Result<Cstr, CstrPtr>;
        fn focus(&self) -> Result<(), CstrPtr>;
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

    #[macro_export]
    macro_rules! impl_TaggedObject_trait {
        ($struct_name:ty) => {
            impl crate::itaggedobject::ITaggedObject for $struct_name {
                fn get_tag(&self) -> crate::Tag {
                    unsafe { crate::jam::JAM_ask_object_tag(self.ptr) }
                }
                fn get_ptr(&self) -> usize {
                    self.ptr
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_UIBlock_trait {
        ($struct_name:ty) => {
            impl crate::blockstyler::uiblocktrait::IUIBlock for $struct_name {
                fn get_enable(&self) -> Result<bool, crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let mut p = false;
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_get_enable(self.ptr, &mut p) };
                    if n != 0 {
                        return Err(unsafe {crate::jam::ERROR_decode(n) });
                    }
                    Ok(p)
                }

                fn set_enable(&self, value: bool) -> Result<(), crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_set_enable(self.ptr, value) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(())
                }

                fn get_expanded(&self) -> Result<bool, crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let mut p = false;
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_get_expanded(self.ptr, &mut p) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(p)
                }

                fn set_expanded(&self, value: bool) -> Result<(), crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_set_expanded(self.ptr, value) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(())
                }

                fn get_group(&self) -> Result<bool, crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let mut p = false;
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_get_group(self.ptr, &mut p) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(p)
                }

                fn set_group(&self, value: bool) -> Result<(), crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_set_group(self.ptr, value) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(())
                }

                fn get_label(&self) -> Result<crate::cstr::Cstr, crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let mut p = 0;
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_get_label(self.ptr, &mut p) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    let str = unsafe { crate::jam::TEXT_to_locale(p) };
                    unsafe { crate::jam::TEXT_free(p as _) };
                    Ok(crate::cstr::Cstr::from_ptr(str, true))
                }

                fn set_label(&self, value: crate::cstr::Cstr) -> Result<(), crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let text = unsafe { crate::jam::TEXT_create_string(value.ptr) };
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_set_label(self.ptr, text as _) };
                    unsafe { crate::jam::TEXT_free(text as _) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(())
                }

                fn get_name(&self) -> Result<crate::cstr::Cstr, crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let mut p = 0;
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_get_name(self.ptr, &mut p) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(crate::cstr::Cstr::from_ptr(p as _, true))
                }

                fn get_show(&self) -> Result<bool, crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let mut p = false;
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_get_show(self.ptr, &mut p) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(p)
                }

                fn set_show(&self, value: bool) -> Result<(), crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_set_show(self.ptr, value) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(())
                }

                fn get_type(&self) -> Result<crate::cstr::Cstr, crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let mut p = 0usize;
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_get_type(self.ptr, &mut p) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(crate::cstr::Cstr::from_ptr(p as _, true))
                }

                fn focus(&self) -> Result<(), crate::CstrPtr> {
                    unsafe { crate::jam::JAM_start_wrapped_call() };
                    let n = unsafe { crate::blockstyler::uiblocktrait::XJA_BLOCK_STYLER_test_focus_change(self.ptr) };
                    if n != 0 {
                        return Err(unsafe { crate::jam::ERROR_decode(n) });
                    }
                    Ok(())
                }
            }
        };
    }
