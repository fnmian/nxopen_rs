use nxopen_rs::blockstyler::blockdialog::{Application};
use nxopen_rs::blockstyler::button::Button;
use nxopen_rs::blockstyler::uiblock::{IUIBlock, UIBlock};
use nxopen_rs::taggedobject::TaggedObject;
use nxopen_rs::{blockstyler::blockdialog::BlockDialog, cstr::NXstr};
use nxopen_rs::{nx_msgboxa, nx_println};

#[unsafe(no_mangle)]
pub extern "C" fn ufusr() {
    let demo01 = Demo01::create_dialog(Demo01::default(),"demo01.dlx".into());
    match demo01 {
        Ok(d) => {
            let _ = d.show();
            ()
        }
        Err(e) => {
            nx_msgboxa!("{e}");
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

impl Application for Demo01 {
    fn initialize(&mut self) -> Result<(), NXstr> {
        self.bt = self.the_dialog.find_block("button0".into())?.into();
        Ok(())
    }
    fn update(&self, block: UIBlock) -> Result<i32, NXstr> {
        if block == self.bt {
           nx_println!("{}",self.bt.block().get_ptr());
        }
        Ok(0)
    }

    fn set_dialog(&mut self, dialog: &BlockDialog) {
        self.the_dialog = dialog.clone();
        self.the_dialog.add_update_initialize();
        self.the_dialog.add_update_handler();
    }
    
    fn create_dialog(self,name: NXstr) -> Result<BlockDialog, NXstr> {
        let dlg = nxopen_rs::blockstyler::blockdialog::create_dialog(Demo01::default(), name)?;
        Ok(dlg)
    }
}
