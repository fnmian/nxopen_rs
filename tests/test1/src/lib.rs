use std::os::raw;

use nxopen_rs::{
    cstr::{AsCstr, Cstr, Tostr},
    nxopen_ui::{
        self,
        block_dialog::{BlockDialog, Response, Update},
        compositeblock::CompositeBlock,
        ui::{self, uc1601},
        uiblock::UIBlock,
    },
    syss,
    utilities::jam::{jam_ask_object_tag, jam_start_wrapped_call},
};

pub struct Demo01 {
    the_dialog: BlockDialog,
}
impl Demo01 {
    pub fn new(name: &Cstr) -> Result<Self, i32> {
        let d = nxopen_ui::ui::UI::create_dialog(name);
        match d {
            Ok(dlg) => Ok({
                let d = Demo01 { the_dialog: dlg };
                d
            }),
            Err(err) => Err(err),
        }
    }
    pub fn show(&self) -> Response {
        self.the_dialog.show()
    }
}
impl Update for Demo01 {
    fn update(&self, bk: usize) -> i32 {
        ui::print(
            &format!("{:x}", self.the_dialog.get_ptr())
                .to_string()
                .as_str()
                .to_cstring(),
        );
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr() -> i32 {
    if let Ok(d) = Demo01::new(&"demo01.dlx".to_cstring()) {
        d.the_dialog.add_initialize_handler(|| {});
        d.the_dialog.add_update_handler(&d);
        d.show();
    }
    else {
        ui::uc1601("a\0".as_ptr(),1);
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
