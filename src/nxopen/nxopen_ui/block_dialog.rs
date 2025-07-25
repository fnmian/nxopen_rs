use std::usize;

use crate::{cstr::Cstr, lazy_load_function, nxopen_ui::ui::{self, UI}, taggedobject::Tag};

pub struct BlockDialog {
    pub(crate) ptr: usize,
}
impl BlockDialog {
    pub fn create_dialog(dialog_name: &Cstr) -> Result<BlockDialog, i32> {
        UI::create_dialog(dialog_name)
    }
    pub fn show(&self) -> Response {
        let mut response: Response = Response::Back;
        x18_ja_block_styler_dialog_show(self.ptr, &mut response);
        response
    }
    pub fn add_initialize_handler(&self, handler: fn()) {
        xja_block_styler_dialog_add_initialize_handler(self.ptr, handler as *const ());
    }
    pub fn add_apply_handler(&self, handler: fn() -> i32) {
        xja_block_styler_dialog_add_apply_handler(self.ptr, handler as *const ());
    }
    pub fn add_cancel_handler(&self, handler: fn() -> i32) {
        xja_block_styler_dialog_add_cancel_handler(self.ptr, handler as *const ());
    }
    pub fn add_close_handler(&self, handler: fn() -> i32) {
        xja_block_styler_dialog_add_close_handler(self.ptr, handler as *const ());
    }
    pub fn add_dialog_shown_handler(&self, handler: fn()) {
        xja_block_styler_dialog_add_dialog_shown_handler(self.ptr, handler as *const ());
    }
    pub fn add_filter_handler(
        &self,
        handler: fn(selectionBlock: usize, selected_object: Tag) -> i32,
    ) {
        xja_block_styler_dialog_add_filter_handler(self.ptr, handler as *const ());
    }
    pub fn add_focus_notify_handler(&self, handler: fn(focusBlock: usize, isFocus: bool) -> i32) {
        xja_block_styler_dialog_add_focus_notify_handler(self.ptr, handler as *const ());
    }
    pub fn add_keyboard_focus_notify_handler(
        &self,
        handler: fn(focusBlock: usize, isFocus: bool) -> i32,
    ) {
        xja_block_styler_dialog_add_keyboard_focus_notify_handler(self.ptr, handler as *const ());
    }
    pub fn add_ok_handler(&self, handler: fn() -> i32) {
        xja_block_styler_dialog_add_ok_handler(self.ptr, handler as *const ());
    }
    pub fn add_update_handler<T>(&self, object: &T)
    where
        T: Update+ 'static,
    {
      let p = self.into_dialog_wrap();
        let update = Box::new(BlockDialogUpdate::new(object));
        p.call_backs.update = Some(update);
    }
    pub fn add_enable_ok_button_handler(&self, handler: fn() -> i8) {
        xja_block_styler_dialog_add_enable_ok_button_handler(self.ptr, handler as *const ());
    }
    pub fn into_dialog_wrap(&self) -> &mut DialogWrap {
       unsafe { &mut *(self.ptr as *mut DialogWrap) }
    }

    pub fn get_ptr(&self) -> usize {
        self.ptr
    }
}

impl TransientObject for BlockDialog {
    fn get_ptr(&self) -> usize {
        self.ptr
    }
}
impl Drop for BlockDialog {
    fn drop(&mut self) {
        xja_block_styler_dialog_dispose(self.ptr);
    }
}

pub trait TransientObject {
    fn get_ptr(&self) -> usize;
}

pub trait Update {
    fn update(&self, bk: usize) -> i32;
}
#[repr(C)]
pub struct BlockDialogUpdate {
    adapter: *mut CallBackAdapter,
    dlg: *const (),
    func: *const (),
}
impl BlockDialogUpdate {
    pub fn new<T>(d: &T) -> Self
    where
        T: Update,
    {
       // ui::print(&Cstr { ptr:format!("{:X}\n\0",d as *const T as usize).as_ptr() as _ });
        BlockDialogUpdate {
            dlg: d as *const T as *const (),
            func: T::update as *const (),
            adapter: Box::into_raw(Box::new(CallBackAdapter {
                free_fn: BlockDialogUpdate::free as *const (),
                clone_fn: std::ptr::null(),
                dynamic_cast_fn: std::ptr::null(),
                adapter_fn: BlockDialogUpdate::update_adapter as *const (),
            })),
        }
    }
    fn update_adapter(&self, block: usize) -> i32 {
        let f: fn(*const (), usize) -> i32 = unsafe { std::mem::transmute(self.func) };
        f(self.dlg, block)
    }
    fn free(&self, _b: bool) -> i32 {
        0
    }
}

pub struct CallBackAdapter {
    #[allow(dead_code)]
    free_fn: *const (),
    #[allow(dead_code)]
    clone_fn: *const (),
    #[allow(dead_code)]
    dynamic_cast_fn: *const (),
    #[allow(dead_code)]
    adapter_fn: *const (),
}

#[repr(C)]
pub enum Response {
    Back = 1,
    Cancel,
    Ok,
    ObjectSelectedByName,
    ObjectSelected,
}
#[repr(C)]
pub struct DialogWrap<'a> {
    vt: usize,
    topblock: usize,
    call_backs: &'a mut DialogCallBacks,
}

#[repr(C)]
pub struct DialogCallBacks {
    ok: usize,
    apply: usize,
    cancel: usize,
    close: usize,
    update: Option<Box<BlockDialogUpdate>>,
    filter: usize,
    initialize: usize,
    shown: usize,
    focus_notify: usize,
    keyboard_focus_notify: usize,
    enable_okbutton: usize,
}

lazy_load_function! {
    pub fn x18_ja_block_styler_dialog_show(dialog: usize, out:&mut Response)-> i32{
        dll: "libnxblockstyler.dll",
        func: "X18JA_BLOCK_STYLER_DIALOG_show"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_dialog_add_initialize_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_initialize_handler"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_dialog_add_apply_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_apply_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_cancel_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_cancel_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_close_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_close_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_dialog_shown_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_dialog_shown_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_filter_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_filter_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_focus_notify_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_focus_notify_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_keyboard_focus_notify_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_keyboard_focus_notify_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_ok_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_ok_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_update_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_add_update_handler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_add_enable_ok_button_handler(dialog: usize, handler: *const()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_AddEnableOKButtonHandler"
    }
}

lazy_load_function! {
    pub fn xja_block_styler_dialog_dispose(dialog: usize) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_dispose"
    }
}
lazy_load_function! {
    pub fn xja_block_styler_dialog_get_top_block(dialog: usize,top_block:*const ()) -> i32{
        dll: "libnxblockstyler.dll",
        func: "XJA_BLOCK_STYLER_DIALOG_get_TopBlock"
    }
}
