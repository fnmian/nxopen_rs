use crate::{impl_UIBlock_trait, impl_TaggedObject_trait, blockstyler::{button::Button}, itaggedobject::ITaggedObject};

#[derive(Clone)]
pub struct UIBlock {
    pub(crate) ptr: usize,
}
impl UIBlock {
   pub fn new(bk:usize)->Self {
        UIBlock { ptr: bk }
    }
}
impl_UIBlock_trait!(UIBlock);
impl_TaggedObject_trait!(UIBlock);

impl PartialEq<Button> for UIBlock {
    fn eq(&self, other: &Button) -> bool {
        self.ptr==other.get_ptr()
    }
}

