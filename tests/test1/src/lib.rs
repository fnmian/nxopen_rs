use nxopen_rs::{ nxopen_ui::ui::{print}};

#[unsafe(no_mangle)]
pub extern "C" fn ufusr() -> i32 {
    print(nxopen_rs::syss::get_release());
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
