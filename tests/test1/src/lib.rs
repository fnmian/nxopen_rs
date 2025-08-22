
use nxopen_rs::blockstyler::blockdialog::{Initialize, Response, Update};
use nxopen_rs::blockstyler::button::Button;
use nxopen_rs::blockstyler::uiblock::UIBlock;
use nxopen_rs::{nx_msgbox_ansi, nx_println};

use nxopen_rs::itaggedobject::ITaggedObject;
use nxopen_rs::{
    blockstyler::blockdialog::BlockDialog, cstr::Cstr, messagebox::DialogType
};

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
#[derive(Clone,Default)]
pub struct Demo01 {
    the_dialog: BlockDialog,
    bt:Button,
}
impl Demo01 {
    pub fn new(name: Cstr) -> Result<Self, String> {
        let d = nxopen_rs::blockstyler::blockdialog::create_dialog(name);
        match d {
            Ok(dlg) => Ok({
                let d = Demo01 { the_dialog: dlg,..Default::default()};
                d.the_dialog.add_update_initialize(d.clone());
                d.the_dialog.add_update_handler(d.clone());
                d
            }),
            Err(err) => Err(err),
        }
    }
    pub fn show(&self) -> Result<Response, String> {
        self.the_dialog.show()
    }
}
impl Initialize for Demo01 {
    fn initialize(&mut self) {
       self.bt=self.the_dialog.find_block("button0".into()).unwrap().into();
       nx_println!("{:x}",self.bt.get_ptr());
    }
}
impl Update for Demo01 {
    fn update(&self, block: UIBlock) -> i32 {
        if block==self.bt {
            nx_println!("666");
        }
        
        0
    }
}
