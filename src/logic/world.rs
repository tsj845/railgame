//! world data

use std::{borrow::Cow, error::Error, fmt, mem};

use serde::Deserialize;

use super::company::CompanyId;

pub const MIC_SCALE: usize = 5;
pub const MIC_ASCALE: usize = MIC_SCALE*MIC_SCALE;

/// micro grid location
pub type SLoc = (u8,u8);

#[derive(Clone, Copy, PartialEq)]
/// macro grid location
pub struct Loc{pub x:u16,pub y:u16}

#[derive(Clone, Copy)]
/// macro and micro together
pub struct FullLoc{pub mac:Loc, pub mic:SLoc}

#[derive(Deserialize, Debug, Clone)]
struct SMSubs {
    heights: [u8;MIC_ASCALE],
    terrains: [u8;MIC_ASCALE],
    #[serde(default)]
    owners: [CompanyId;MIC_ASCALE]
}
impl Default for SMSubs {
    fn default() -> Self {
        Self { heights: [0;MIC_ASCALE], terrains: [0;MIC_ASCALE], owners: [0;MIC_ASCALE] }
    }
}
#[derive(Deserialize, Debug, Clone)]
struct SMac {
    subs:SMSubs,
    #[serde(default)]
    free_rail:bool,
    #[serde(default)]
    owner:CompanyId
}
impl Default for SMac {
    fn default() -> Self {
        Self{subs:SMSubs::default(),free_rail:true,owner:0}
    }
}
#[derive(Deserialize, Debug)]
struct SWorld {
    dimensions: (u16,u16),
    tiles: Cow<'static,[SMac]>
}

#[derive(Debug)]
pub struct World {
    /// macro scale grid
    pub grid: Box<[MacCell]>,
    pub width: u16,
    /// in-game time
    pub time: (),
    /// play time
    pub duration: (),
}
impl World {
    pub fn from_jsonstr(jsonstr: &'static str) -> Self {
        let jsw: SWorld = serde_json::from_str(jsonstr).unwrap();
        println!("{:?}", &jsw);
        let tilecount = (jsw.dimensions.0*jsw.dimensions.1) as usize;
        let mut mcells: Vec<MacCell> = Vec::with_capacity(tilecount);
        for i in 0..tilecount {
            let sm = &jsw.tiles[i];
            // using mem::zeroed here is fine because it's just producing placeholder values
            let mut msubs: [MicCell; MIC_ASCALE] = unsafe{[mem::zeroed();MIC_ASCALE]};
            for j in 0..MIC_ASCALE {
                msubs[j] = MicCell{land_owner:sm.subs.owners[j],elevation:sm.subs.heights[j],terrain:Terrain::try_from(sm.subs.terrains[j]).unwrap()};
            }
            mcells.push(MacCell{subs:msubs,free_rail:sm.free_rail,land_owner:sm.owner});
        }
        Self {grid:mcells.into_boxed_slice(),width:jsw.dimensions.0,time:(),duration:()}
    }
}
#[derive(Debug)]
pub struct MacCell {
    /// micro scale grid
    pub subs: [MicCell; MIC_ASCALE],
    /// flag for if the play must go into micro view to build rail here
    pub free_rail: bool,
    pub land_owner: CompanyId,
}
#[derive(Debug, Clone, Copy)]
pub struct MicCell {
    pub land_owner: CompanyId,
    pub elevation: u8,
    pub terrain: Terrain,
}
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Terrain {
    Water = 0,
    Grass,
    Field,
    Rock,
    Forest
}
#[derive(Debug)]
pub struct CoercionError {
    msg: &'static str,
}
impl fmt::Display for CoercionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Coercion Error: {}", &self.msg))
    }
}
impl Error for CoercionError {}
impl CoercionError {
    pub fn new(msg: &'static str) -> Self {
        Self{msg}
    }
}
impl TryFrom<u8> for Terrain {
    type Error = CoercionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 4 {
            return Err(CoercionError::new("invalid terrain id"));
        }
        return Ok([Terrain::Water,Terrain::Grass,Terrain::Field,Terrain::Rock,Terrain::Forest][value as usize]);
    }
}
// pub type Terrain = u8;
// #[allow(non_upper_case_globals)]
// pub const Water: Terrain = 0;
// #[allow(non_upper_case_globals)]
// pub const Grass: Terrain = 1;
// #[allow(non_upper_case_globals)]
// pub const Field: Terrain = 2;
// #[allow(non_upper_case_globals)]
// pub const Rock: Terrain = 3;
// #[allow(non_upper_case_globals)]
// pub const Forest: Terrain = 4;

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
