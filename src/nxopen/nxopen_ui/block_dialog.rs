use crate::{cstr::Cstr, lazy_load_function, nxopen_ui::ui::UI, taggedobject::Tag};

pub struct BlockDialog {
    pub(crate) ptr: usize,
}
impl BlockDialog {
    pub fn create_dialog(dialog_name: &Cstr) -> Result<BlockDialog,*const u8> {
       UI::create_dialog(dialog_name)
    }
    pub fn show(&self) -> Response {
        let mut response: Response = Response::Back;
        x18_ja_block_styler_dialog_show(self.ptr, &mut response);
        response
    }
    pub fn add_initialize_handler(&self, handler:fn()) {
        xja_block_styler_dialog_add_initialize_handler(self.ptr, handler as *const ());
    }
    pub fn add_apply_handler(&self, handler:fn()->i32) {
        xja_block_styler_dialog_add_apply_handler(self.ptr, handler as *const ());
    }
    pub fn add_cancel_handler(&self, handler:fn()->i32) {
        xja_block_styler_dialog_add_cancel_handler(self.ptr, handler as *const ());
    }
    pub fn add_close_handler(&self, handler:fn()->i32) {
        xja_block_styler_dialog_add_close_handler(self.ptr, handler as *const ());
    }
    pub fn add_dialog_shown_handler(&self, handler:fn()) {
        xja_block_styler_dialog_add_dialog_shown_handler(self.ptr, handler as *const ());
    }
    pub fn add_filter_handler(&self, handler:fn(selectionBlock:usize,selected_object:Tag)->i32) {
        xja_block_styler_dialog_add_filter_handler(self.ptr, handler as *const ());
    }
    pub fn add_focus_notify_handler(&self, handler:fn(focusBlock:usize,isFocus:bool)->i32) {
        xja_block_styler_dialog_add_focus_notify_handler(self.ptr, handler as *const ());
    }
    pub fn add_keyboard_focus_notify_handler(&self, handler:fn(focusBlock:usize,isFocus:bool)->i32) {
        xja_block_styler_dialog_add_keyboard_focus_notify_handler(self.ptr, handler as *const ());
    }
    pub fn add_ok_handler(&self, handler:fn()->i32) {
        xja_block_styler_dialog_add_ok_handler(self.ptr, handler as *const ());
    }
    pub fn add_update_handler(&self, handler:fn(styler_item:usize)->i32) {
        xja_block_styler_dialog_add_update_handler(self.ptr, handler as *const ());
    }
    pub fn add_enable_ok_button_handler(&self, handler:fn()->i8) {
        xja_block_styler_dialog_add_enable_ok_button_handler(self.ptr, handler as *const ());
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
#[repr(C)]
pub enum Response
{
	Back = 1,
	Cancel,
	Ok,
	ObjectSelectedByName,
	ObjectSelected
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
