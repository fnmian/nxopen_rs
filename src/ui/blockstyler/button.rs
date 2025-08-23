use crate::blockstyler::uiblock::{IUIBlock, UIBlock};




#[derive(Clone,Default)]
pub struct Button{
    ptr:usize,
}
impl Button {
    pub fn new(btptr:usize)->Self {
        Button { ptr: btptr }
    }

}

impl From<UIBlock> for Button {
    fn from(value: UIBlock) -> Self {
        Button { ptr: value.ptr }
    }
}
impl IUIBlock for Button {
    fn uiblock(&self)->UIBlock {
        UIBlock { ptr: self.ptr }
    }
}