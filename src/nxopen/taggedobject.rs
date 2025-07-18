
pub type Tag = u32;

pub trait TaggedObject
{
    fn get_tag(&self) -> Tag;
    fn get_ptr(&self) -> usize;
}
