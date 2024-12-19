//! all that has to do with tracks in world space

use super::specs::SpecId;
use std::rc::Weak;

use super::{company::CompanyId, world::{FullLoc, Loc}};

pub struct Track<'a> {
    owner: CompanyId,
    start: FullLoc,
    end:   FullLoc,
    route: &'a[FullLoc]
}
impl<'a> Track<'a> {
    pub fn new(owner: CompanyId, start: FullLoc, end: FullLoc, route: &'a[FullLoc]) -> Self {
        Self{owner,start,end,route}
    }
    pub fn from_locs(owner: CompanyId, locs: &'a[FullLoc]) -> Self {
        // Self{start:locs[0],end:locs[locs.len()-1],route:Self::simplify(locs)}
        Self{owner,start:locs[0],end:locs[locs.len()-1],route:&locs[1..locs.len()-1]}
    }
    // pub fn simplify(locs: &'a[FullLoc]) -> &'a[FullLoc] {
    //     let mut v = Vec::new();
    //     for i in 1..locs.len()-1 {
    //         if locs[i].
    //     }
    // }
}

pub struct Route<'a> {
    owner:  CompanyId,
    name:   &'a str,
    stops:  &'a[FullLoc],
    tracks: &'a[Track<'a>]
}

pub struct Train<'a> {
    owner: CompanyId,
    name:  &'a str,
    route: Route<'a>,
    cars:  &'a[TrainCar<'a>],
    spec:  SpecId,
}
pub struct TrainCar<'a> {
    owner: CompanyId,
    train: Weak<Train<'a>>,
    // cargo: Cargo,
    quant: u16,
    // capacity: u16,
    spec:  SpecId,
}

/// Cargo types, all negative entries are placeholders
pub enum Cargo {
    Passengers = 0,
    Oil,
    Coal,
    Grain,
    Wood,
    Textile,
    Livestock,
    Gasoline,
    Mail,
    Munitions,
    Weapons,
    Metal = -10,
    LivestockProducts,
    IntermediaryGoods,
    ProductivityBoosters,
    Substrates,
}

// fn t() {
//     Track::new(0,FullLoc::from(Loc{x:0,y:0}), FullLoc::from(Loc{x:0,y:0}), vec![FullLoc::from(Loc{x:0,y:0})].as_slice());
// }
