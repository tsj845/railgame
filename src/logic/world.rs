//! world data

#[derive(Clone, Copy)]
pub struct Loc{pub x:u16,pub y:u16}
#[derive(Clone, Copy)]
pub struct FullLoc{pub mac:Loc, pub mic:(u8,u8)}
impl FullLoc{pub fn new(mac:Loc, mic:(u8,u8))->Self{Self{mac,mic}}}
impl From<Loc> for FullLoc {
    fn from(value: Loc) -> Self {
        Self {mac:value, mic:(255,255)}
    }
}
pub struct World<'a> {
    /// macro scale grid
    grid: &'a mut [MacCell<'a>],
    /// in-game time
    time: (),
    /// play time
    duration: ()
}
pub struct MacCell<'a> {
    /// micro scale grid
    subs: &'a mut [MicCell]
}
pub struct MicCell {}
