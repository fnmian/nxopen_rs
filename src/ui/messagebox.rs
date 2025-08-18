use crate::{
    cstr::Cstr,
    jam::{self, JAM_start_wrapped_call},
};

#[repr(C)]
pub enum DialogType {
    Error,
    Warning,
    Information,
    Question,
}

pub fn show(title: Cstr, msgbox_type: DialogType, message: Cstr) -> Result<i32,Cstr> {
    unsafe { JAM_start_wrapped_call() };
    let result = 0;
    let n = unsafe { X0JA_NXMESSAGE_BOX_show(title.ptr, msgbox_type, message.ptr, &result) };
    if n != 0 {
        return Err(Cstr::from_ptr(unsafe { jam::ERROR_decode(n) }));
    }
    Ok(result)
}

#[link(name = "libgeomint", kind = "dylib")]
unsafe extern "C" {
    fn X0JA_NXMESSAGE_BOX_show(
        title: *const u8,
        msgboxType: DialogType,
        message: *const u8,
        result: &i32,
    ) -> i32;
}
