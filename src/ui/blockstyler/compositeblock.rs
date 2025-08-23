use crate::blockstyler::uiblock::{IUIBlock, UIBlock};


#[derive(Clone)]
pub struct CompositeBlock {
    pub(crate) ptr: usize,
}
impl IUIBlock for CompositeBlock {
    fn uiblock(&self)->UIBlock {
        UIBlock { ptr: self.ptr }
    }
}
