use nxopen_rs::{cstr::{AsCstr, Cstr}, nxopen_ui::ui};

#[unsafe(no_mangle)]
pub extern "C" fn ufusr() -> i32 {
    match create_dialog(&"Demo01.dlx".to_cstring()) {
        Ok(_) => 0,
        Err(err) => {
            ui::uc1601(err, 1);
            0
        }
    }
}

fn create_dialog(dialog_name: &Cstr) -> Result<i32, *const u8> {
    let d = ui::UI::create_dialog(dialog_name)?;
    d.add_initialize_handler(|| {});
    d.add_update_handler(|item| {
       ui::print(&(format!("item:{:X}\n",item)).as_str().to_cstring());
        0
    });
    d.show();
    Ok(0)
}



#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
