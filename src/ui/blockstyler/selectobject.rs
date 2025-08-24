use crate::blockstyler::uiblock::{IUIBlock, UIBlock};




#[derive(Clone,Default)]
pub struct SelectObject{
    ptr:usize,
}
impl SelectObject {
    pub fn new(btptr:usize)->Self {
        SelectObject { ptr: btptr }
    }

}

impl From<UIBlock> for SelectObject {
    fn from(value: UIBlock) -> Self {
        SelectObject { ptr: value.ptr }
    }
}
impl IUIBlock for SelectObject {
    fn block(&self)->UIBlock {
        UIBlock { ptr: self.ptr }
    }
}