
use crate::cstr::Cstr;
use crate::utilities::jam::JAM_start_wrapped_call;
use crate::{debugbreak, jam,p_ss};

#[derive(Clone)]
pub struct BlockDialog {
   ptr: usize,
}


impl BlockDialog {
    pub fn show(&self) -> Result<Response, String> {
        let mut response: Response = Response::Back;
        let num = unsafe { X18JA_BLOCK_STYLER_DIALOG_show(self.get_ptr(), &mut response) };
        if num != 0 {
            return Err(p_ss!(jam::ERROR_decode(num)));
        }
        Ok(response)
    }
    pub fn get_ptr(&self) -> usize {
        self.ptr
    }
    pub(crate) fn into_dialog_wrap(&self) -> &mut DialogWrap {
       unsafe { &mut *(self.get_ptr() as *mut DialogWrap) }
    }
    pub fn add_update_handler(&self, app: impl Update + 'static) {
       self.into_dialog_wrap().call_backs.update=Some(Box::new(BlockDialogUpdateAdapter::new(app)))
    }
    pub fn add_update_initialize(&self, app: impl Initialize + 'static) {
       self.into_dialog_wrap().call_backs.initialize=Some(Box::new(BlockDialogInitializeAdapter::new(app)))
    }
}
    pub fn create_dialog(dialog_name: &Cstr) -> Result<BlockDialog, String> {
        let mut dialog: usize = 0;
        unsafe { JAM_start_wrapped_call() };
        let errcode = unsafe { XJA_UI_MAIN_create_dialog(dialog_name.ptr, &mut dialog) };
        if errcode != 0 {
            return Err(p_ss!(jam::ERROR_decode(errcode)));
        }
        Ok(BlockDialog {ptr: dialog })
    }
struct CallBackAdapter {
    _free: usize,
    _clone: usize,
    _equal: usize,
    _adapter: usize,
}

struct BlockDialogUpdateAdapter {
    _vt: Box<CallBackAdapter>,
    data: Option<Box<dyn Update>>,
}
impl BlockDialogUpdateAdapter {
    pub(crate) fn new(app: impl Update + 'static) -> Self {
        BlockDialogUpdateAdapter {
            _vt: Box::new(CallBackAdapter {
                _free: BlockDialogUpdateAdapter::free as *const () as usize,
                _clone: 0,
                _equal: 0,
                _adapter: BlockDialogUpdateAdapter::adapter as *const () as usize,
            }),
            data: Some(Box::new(app)),
        }
    }
    fn free(&self){

    }
    fn adapter(&self, styler_item: usize) -> i32 {
        match &self.data {
            Some(d) => d.update(styler_item),
            None => {
                debugbreak();
                0
            }
        }
    }
}


struct BlockDialogInitializeAdapter {
    _vt: Box<CallBackAdapter>,
    data: Option<Box<dyn Initialize>>,
}
impl BlockDialogInitializeAdapter {
    pub(crate) fn new(app: impl Initialize + 'static) -> Self {
        BlockDialogInitializeAdapter {
            _vt: Box::new(CallBackAdapter {
                _free: BlockDialogInitializeAdapter::free as *const () as usize,
                _clone: 0,
                _equal: 0,
                _adapter: BlockDialogInitializeAdapter::adapter as *const () as usize,
            }),
            data: Some(Box::new(app)),
        }
    }
    fn free(&self) {
        
    }
    fn adapter(&self){
        match &self.data {
            Some(d) => d.initialize(),
            None => {
                debugbreak();
            }
        }
    }
}

#[repr(C)]
pub(crate) struct DialogWrap<'a> {
    vt: usize,
    topblock: usize,
    call_backs: &'a mut DlgCallback,
}
#[derive(Default)]
#[repr(C)]
struct DlgCallback {
    ok: usize,
    apply: usize,
    cancel: usize,
    close: usize,
    update: Option<Box<BlockDialogUpdateAdapter>>,
    filter: usize,
    initialize: Option<Box<BlockDialogInitializeAdapter>>,
    shown: usize,
    focus_notify: usize,
    keyboard_focus_notify: usize,
    enable_okbutton: usize,
}
impl Drop for BlockDialog {
    fn drop(&mut self) {
        if self.ptr != 0 {
            unsafe {
                XJA_BLOCK_STYLER_DIALOG_dispose(self.ptr);
            }
           self.ptr=0;
        }
    }
}

#[repr(C)]
pub enum Response {
    Back = 1,
    Cancel,
    Ok,
    ObjectSelectedByName,
    ObjectSelected,
}

#[link(name = "libgeomint", kind = "dylib")]
unsafe extern "C" {
    pub fn XJA_UI_MAIN_create_dialog(dialog_name: *const u8, dialog: &mut usize) -> i32;
}
#[link(name = "libnxblockstyler", kind = "dylib")]
unsafe extern "C" {
    pub fn X18JA_BLOCK_STYLER_DIALOG_show(dialog: usize, out: &mut Response) -> i32;
    pub fn XJA_BLOCK_STYLER_DIALOG_get_TopBlock(dialog: usize, top_block: *const ()) -> i32;
    pub fn XJA_BLOCK_STYLER_DIALOG_dispose(dialog: usize) -> i32;
}

pub trait Update {
    fn update(&self, styler_item: usize) -> i32;
}
pub trait Initialize {
    fn initialize(&self);
}