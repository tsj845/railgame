//! support for saving the game

pub trait Saveable {
    fn to_bytes(&self) -> &[u8];
    fn from_bytes(&self, b: &[u8], p: &mut usize) -> ();
}

pub struct SaveMeta {
    //
}

pub struct RawSave {
    buf: Vec<u8>
}
impl RawSave {
    pub fn new() -> Self{Self{buf:Vec::new()}}
    pub fn add(&mut self, obj: impl Saveable) -> () {
        self.buf.extend_from_slice(obj.to_bytes());
    }
}
