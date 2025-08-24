use crate::blockstyler::compositeblock::CompositeBlock;
use crate::blockstyler::uiblock::UIBlock;
use crate::cstr::NXstr;
use crate::utilities::jam::JAM_start_wrapped_call;
use crate::{CstrPtr, jam, list_ui};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct BlockDialog {
    ptr: Option<usize>,
    app: Option<Rc<RefCell<dyn Application>>>,
}

impl BlockDialog {
    pub fn show(&self) -> Result<Response, NXstr> {
        unsafe { JAM_start_wrapped_call() };
        let mut response: Response = Response::Back;
        let num = unsafe { X18JA_BLOCK_STYLER_DIALOG_show(self.get_ptr(), &mut response) };
        if num != 0 {
            return Err(NXstr::from_nxstr_ptr(
                unsafe { jam::ERROR_decode(num) },
                false,
            ));
        }
        Ok(response)
    }
    pub fn get_ptr(&self) -> usize {
        match self.ptr {
            Some(p) => p,
            None => {
                panic!("ptr=0\0")
            }
        }
    }
    pub(crate) fn into_dialog_wrap(&self) -> &mut DialogWrap {
        unsafe { &mut *(self.get_ptr() as *mut DialogWrap) }
    }
    pub fn add_update_handler(&self) {
        self.into_dialog_wrap().call_backs.update =
            Some(Box::new(BlockDialogUpdateAdapter::new(self.app.clone())))
    }
    pub fn add_update_initialize(&self) {
        self.into_dialog_wrap().call_backs.initialize = Some(Box::new(
            BlockDialogInitializeAdapter::new(self.app.clone()),
        ))
    }
     pub fn add_apply_initialize(&self) {
        self.into_dialog_wrap().call_backs.apply = Some(Box::new(
            BlockDialogApplyAdapter::new(self.app.clone()),
        ))
    }
    pub fn get_topblock(&self) -> Result<CompositeBlock, NXstr> {
        unsafe {
            JAM_start_wrapped_call();
            let mut p = 0;
            let n = XJA_BLOCK_STYLER_DIALOG_get_TopBlock(self.get_ptr(), &mut p);
            if n != 0 {
                return Err(NXstr::from_nxstr_ptr(jam::ERROR_decode(n), false));
            }
            Ok(CompositeBlock { ptr: p })
        }
    }
    pub fn find_block(&self, block_name: NXstr) -> Result<UIBlock, NXstr> {
        unsafe {
            JAM_start_wrapped_call();
            let mut p = 0;
            let n = XJA_BLOCK_STYLER_find_block(
                self.get_topblock().unwrap().ptr,
                block_name.ptr,
                &mut p,
            );
            if n != 0 {
                return Err(NXstr::from_nxstr_ptr(jam::ERROR_decode(n), false));
            }
            if p == 0 {
                return Err(format!("{} not found", block_name).into());
            }
            Ok(UIBlock { ptr: p })
        }
    }
}
pub fn create_dialog(
    app: impl Application + 'static,
    dialog_name: NXstr,
) -> Result<BlockDialog, NXstr> {
    let mut dialog: usize = 0;
    unsafe { JAM_start_wrapped_call() };
    let errcode = unsafe { XJA_UI_MAIN_create_dialog(dialog_name.ptr, &mut dialog) };
    if errcode != 0 {
        return Err(NXstr::from_nxstr_ptr(
            unsafe { jam::ERROR_decode(errcode) },
            false,
        ));
    }
    let block_dialog = BlockDialog {
        ptr: Some(dialog),
        app: Some(Rc::new(RefCell::new(app))),
    };
    if let Some(app_rc) = &block_dialog.app {
        if let Ok(mut app_ref) = app_rc.try_borrow_mut() {
            app_ref.set_dialog(&block_dialog);
        }
    }
    Ok(block_dialog)
}
struct CallBackAdapter {
    _free: usize,
    _clone: usize,
    _equal: usize,
    _adapter: usize,
}

struct BlockDialogUpdateAdapter {
    _vt: Box<CallBackAdapter>,
    data: Option<Rc<RefCell<(dyn Application + 'static)>>>,
}
impl BlockDialogUpdateAdapter {
    pub(crate) fn new(app: Option<Rc<RefCell<(dyn Application + 'static)>>>) -> Self {
        BlockDialogUpdateAdapter {
            _vt: Box::new(CallBackAdapter {
                _free: BlockDialogUpdateAdapter::free as *const () as usize,
                _clone: 0,
                _equal: 0,
                _adapter: BlockDialogUpdateAdapter::adapter as *const () as usize,
            }),
            data: app,
        }
    }
    fn free(&self) {}
    fn adapter(&self, styler_item: usize) -> i32 {
        match &self.data {
            Some(d) => match d.borrow().update(UIBlock { ptr: styler_item }) {
                Ok(ok) => ok,
                Err(e) => {
                    unsafe { list_ui::list_uiprintf("%s\n\0".as_ptr(), e.ptr) };
                    0
                }
            },
            None => {
                panic!()
            }
        }
    }
}

struct BlockDialogInitializeAdapter {
    _vt: Box<CallBackAdapter>,
    data: Option<Rc<RefCell<(dyn Application + 'static)>>>,
}
impl BlockDialogInitializeAdapter {
    pub(crate) fn new(app: Option<Rc<RefCell<(dyn Application + 'static)>>>) -> Self {
        BlockDialogInitializeAdapter {
            _vt: Box::new(CallBackAdapter {
                _free: BlockDialogInitializeAdapter::free as *const () as usize,
                _clone: 0,
                _equal: 0,
                _adapter: BlockDialogInitializeAdapter::adapter as *const () as usize,
            }),
            data: app,
        }
    }
    fn free(&self) {}
    fn adapter(&mut self) {
        match &mut self.data {
            Some(d) => match d.borrow_mut().initialize() {
                Ok(_) => (),
                Err(e) => {
                    unsafe { list_ui::list_uiprintf("%s\n\0".as_ptr(), e.ptr) };
                    ()
                }
            },
            None => {
                panic!()
            }
        }
    }
}

struct BlockDialogApplyAdapter {
    _vt: Box<CallBackAdapter>,
    data: Option<Rc<RefCell<(dyn Application + 'static)>>>,
}
impl BlockDialogApplyAdapter {
    pub(crate) fn new(app: Option<Rc<RefCell<(dyn Application + 'static)>>>) -> Self {
        BlockDialogApplyAdapter {
            _vt: Box::new(CallBackAdapter {
                _free: BlockDialogApplyAdapter::free as *const () as usize,
                _clone: 0,
                _equal: 0,
                _adapter: BlockDialogApplyAdapter::adapter as *const () as usize,
            }),
            data: app,
        }
    }
    fn free(&self) {}
    fn adapter(&mut self)->i32 {
        match &mut self.data {
            Some(d) => match d.borrow_mut().apply() {
                Ok(i) => i,
                Err(e) => {
                    unsafe { list_ui::list_uiprintf("%s\n\0".as_ptr(), e.ptr) };
                    0
                }
            },
            None => {
                panic!()
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
    apply: Option<Box<BlockDialogApplyAdapter>>,
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
        if self.ptr != None {
            unsafe {
                JAM_start_wrapped_call();
                XJA_BLOCK_STYLER_DIALOG_dispose(self.ptr.unwrap());
            }
            self.ptr = None;
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
    pub fn XJA_BLOCK_STYLER_DIALOG_get_TopBlock(dialog: usize, top_block: &mut usize) -> i32;
    pub fn XJA_BLOCK_STYLER_DIALOG_dispose(dialog: usize) -> i32;
    pub fn XJA_BLOCK_STYLER_find_block(
        dialog: usize,
        block_name: CstrPtr,
        block: &mut usize,
    ) -> i32;
}

pub trait Application: Any + 'static {
    fn create_dialog(self, name: NXstr) -> Result<BlockDialog, NXstr>;
    fn set_dialog(&mut self, dialog: &BlockDialog);
    fn initialize(&mut self) -> Result<(), NXstr> {
        Ok(())
    }
    fn update(&self, block: UIBlock) -> Result<i32, NXstr> {
        let _ = block;
        Ok(0)
    }
    fn ok(&self)->Result<i32,NXstr>{Ok(0)}
    fn dialog_show(&self)->Result<(),NXstr>{Ok(())}
    fn apply(&self)->Result<i32,NXstr>{Ok(0)}
    fn cancel(&self)->Result<i32,NXstr>{Ok(0)}
    fn close(&self)->Result<i32,NXstr>{Ok(0)}
    fn enable_ok_button(&self)->Result<bool,NXstr>{Ok(true)}
    fn focus_notify(&self, focus_block: UIBlock,isfocus:bool) -> Result<(), NXstr> {
        let _ = focus_block;
        let _ = isfocus;
        Ok(())
    }
    fn keyboard_focus_notify(&self, block: UIBlock,isfocus:bool) -> Result<(), NXstr> {
        let _ = block;
        let _ = isfocus;
        Ok(())
    }
}
