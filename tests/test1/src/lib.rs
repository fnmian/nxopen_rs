
use nxopen_rs::{blockstyler::blockdialog::BlockDialog, cstr::Cstr, messagebox::DialogType, nx_msgbox, nx_println, };
use nxopen_rs::blockstyler::blockdialog::{Initialize, Response, Update};

#[unsafe(no_mangle)]
pub extern "C" fn ufusr(){
   let demo01 = Demo01::new("demo01.dlx".into());
   match demo01 {
    Ok(d) => {let _ = d.show();()},
    Err(e) => {nx_msgbox!("",DialogType::Information,"{}",e);()},
   }
    
}

#[unsafe(no_mangle)]
pub extern "C" fn ufusr_ask_unload() -> i32 {
    1
}
#[derive(Clone)]
pub struct Demo01 {
    the_dialog: BlockDialog
}
impl Demo01 {
    pub fn new(name:Cstr) -> Result<Self, String> {
        let d = nxopen_rs::blockstyler::blockdialog::create_dialog(&name);
        match d {
            Ok(dlg) => Ok({
                let d = Demo01 {the_dialog: dlg };
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
    fn initialize(&self) {
        
    }
}
impl Update for Demo01 {
    fn update(&self, styler_item: usize) -> i32 {
        nx_println!("{:x}",styler_item);
        0
    }
}