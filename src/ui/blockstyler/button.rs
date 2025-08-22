use crate::{blockstyler::uiblock::UIBlock, impl_TaggedObject_trait, impl_UIBlock_trait};




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

impl_UIBlock_trait!(Button);
impl_TaggedObject_trait!(Button);