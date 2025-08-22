use crate::Tag;

pub trait ITaggedObject {
    fn get_tag(&self)->Tag;
    fn get_ptr(&self)->usize;
}