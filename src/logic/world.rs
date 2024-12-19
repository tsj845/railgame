//! world data

pub const MIC_SCALE: usize = 5;

/// micro grid location
pub type SLoc = (u8,u8);

#[derive(Clone, Copy, PartialEq)]
/// macro grid location
pub struct Loc{pub x:u16,pub y:u16}

#[derive(Clone, Copy)]
/// macro and micro together
pub struct FullLoc{pub mac:Loc, pub mic:SLoc}

pub struct World<'a> {
    /// macro scale grid
    grid: &'a mut [MacCell],
    /// in-game time
    time: (),
    /// play time
    duration: (),
}
pub struct MacCell {
    /// micro scale grid
    subs: [MicCell; MIC_SCALE * MIC_SCALE],
    /// flag for if the play must go into micro view to build rail here
    free_rail: bool,
    land_owner: u16,
}
#[derive(Clone, Copy)]
pub struct MicCell {
    land_owner: u16,
    elevation: u8,
    terrain: Terrain,
}
pub type Terrain = u8;
#[allow(non_upper_case_globals)]
pub const Water: Terrain = 0;
#[allow(non_upper_case_globals)]
pub const Grass: Terrain = 1;
#[allow(non_upper_case_globals)]
pub const Field: Terrain = 2;
#[allow(non_upper_case_globals)]
pub const Rock: Terrain = 3;
#[allow(non_upper_case_globals)]
pub const Forest: Terrain = 4;

impl FullLoc{pub fn new(mac:Loc, mic:SLoc)->Self{Self{mac,mic}}}
impl Loc{pub fn to_index(&self, w: usize) -> usize{self.x as usize+w*self.y as usize}}
impl From<Loc> for FullLoc {
    fn from(value: Loc) -> Self {
        Self {mac:value, mic:(255,255)}
    }
}
impl PartialEq<Loc> for FullLoc {
    fn eq(&self, other: &Loc) -> bool {
        other == self.mac
    }
}
impl PartialEq<Loc> for &Loc {
    fn eq(&self, other: &Loc) -> bool {
        other == *self
    }
}
