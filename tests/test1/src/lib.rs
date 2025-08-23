
use nxopen_rs::blockstyler::blockdialog::{Application, Response};
use nxopen_rs::blockstyler::button::Button;
use nxopen_rs::blockstyler::uiblock::{IUIBlock, UIBlock};
use nxopen_rs::taggedobject::TaggedObject;
use nxopen_rs::{blockstyler::blockdialog::BlockDialog, cstr::Cstr, messagebox::DialogType};
use nxopen_rs::{nx_msgbox_ansi, nx_println};

#[unsafe(no_mangle)]
pub extern "C" fn ufusr() {
    let demo01 = Demo01::new("demo01.dlx".into());
    match demo01 {
        Ok(d) => {
            let _ = d.show();
            ()
        }
        Err(e) => {
            nx_msgbox_ansi!("", DialogType::Information, "{}", e);
            ()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
#[derive(Clone, Default)]
pub struct Demo01 {
    the_dialog: BlockDialog,
    bt: Button,
}
impl Demo01 {
    pub fn new(name: Cstr) -> Result<Self, String> {
        let d = nxopen_rs::blockstyler::blockdialog::create_dialog(Demo01::default(), name);
        match d {
            Ok(dlg) => Ok({
                let d1 = dlg.get_app().unwrap();
                let mut d2 = d1.borrow_mut();
                let d = d2.as_any().downcast_mut::<Demo01>().unwrap();
                d.the_dialog = dlg;
                d.the_dialog.add_update_initialize();
                d.the_dialog.add_update_handler();
                d.clone()
            }),
            Err(err) => Err(err),
        }
    }
    pub fn show(&self) -> Result<Response, String> {
        self.the_dialog.show()
    }
}
impl Application for Demo01 {
    fn initialize(&mut self) {
        self.bt = self.the_dialog.find_block("button0".into()).unwrap().into();
    }
    fn update(&self, block: UIBlock) -> i32 {
        if block == self.bt {
            let _ = self.bt.uiblock().set_show(false);
        }
        0
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
