use nxopen_rs::cstr::Cstr;



#[unsafe(no_mangle)]
pub extern "C" fn ufusr() -> i32 {
   nxopen_rs::ui::UI::uc1601("更何况反馈".to_cstring(), 1);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
