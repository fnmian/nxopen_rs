use nxopen_rs::cstr::{CstrTrait};


#[unsafe(no_mangle)]
pub extern "C" fn ufusr() -> i32 {
    let s = "加分加分打电话的".to_cstring();
   let s2 =&s+&"555".to_cstring();
    nxopen_rs::nxopen_ui::ui::uc1601("加分加分打电话的".to_ansi().ptr, 1);
    nxopen_rs::nxopen_ui::ui::print(&s2);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
