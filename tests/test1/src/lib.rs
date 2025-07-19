use nxopen_rs::{ cstr::Cstr};

#[unsafe(no_mangle)]
pub extern "C" fn ufusr() -> i32 {
    nxopen_rs::nxopen_ui::ui::print("加分加分打电话的".to_cstring());
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
