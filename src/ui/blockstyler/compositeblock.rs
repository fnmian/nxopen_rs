use crate::{impl_TaggedObject_trait, impl_UIBlock_trait};


#[derive(Clone)]
pub struct CompositeBlock {
    pub(crate) ptr: usize,
}
impl CompositeBlock {
    
}
impl_TaggedObject_trait!(CompositeBlock);
impl_UIBlock_trait!(CompositeBlock);