use nxopen_rs::{
    cstr::Cstr,nx_println, nxopen_ui::{
        self,
        block_dialog::{BlockDialog, Response, Update},
        ui::{self},
        uiblock::UIBlock,
    }, syss, taggedobject::TaggedObject
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
    pub fn init(&self) {
        self.the_dialog.add_initialize_handler(|| {});
        self.the_dialog.add_update_handler(self);
    }
    pub fn show(&self) -> Response {
        self.the_dialog.show()
    }
}
impl Update for Demo01 {
    fn update(&self, bk: usize) -> i32 {
        match self.the_dialog.get_topblock() {
            Ok(o) => nx_println!("{}", o.get_tag()),
            Err(e) => nx_println!("{:p}", syss::decode_error(e)),
        }
        nx_println!("{:X}", bk);
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr() -> i32 {
    if let Ok(d) = Demo01::new(&"demo01.dlx".into()) {
        d.init();
        d.show();
    } else {
        ui::uc1601("a\0".as_ptr(), 1);
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
