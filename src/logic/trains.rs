//! all that has to do with tracks in world space

use super::specs::{CarSpec, SpecId};
use std::{rc::Weak, sync::{Arc, LazyLock, Mutex}};

use super::{company::CompanyId, world::FullLoc};

pub type TrackId = u32;
pub type RouteId = u32;
pub type TrainId = u32;
pub type CarId = u32;

static TRACK_ID_COUNTER: LazyLock<Arc<Mutex<u32>>> = LazyLock::new(||Arc::new(Mutex::new(0u32)));

/// gets the next track id, threadsafe
fn get_next_trackid() -> TrackId {
    let mut l = TRACK_ID_COUNTER.lock().unwrap();
    let r = *l;
    *l += 1;
    return r;
}

pub struct Track<'a> {
    id:    TrackId,
    owner: CompanyId,
    start: FullLoc,
    end:   FullLoc,
    route: &'a[FullLoc]
}
impl<'a> Track<'a> {
    pub fn new(owner: CompanyId, start: FullLoc, end: FullLoc, route: &'a[FullLoc]) -> Self {
        Self{id:get_next_trackid(),owner,start,end,route}
    }
    pub fn from_locs(owner: CompanyId, locs: &'a[FullLoc]) -> Self {
        // Self{start:locs[0],end:locs[locs.len()-1],route:Self::simplify(locs)}
        Self{id:get_next_trackid(),owner,start:locs[0],end:locs[locs.len()-1],route:&locs[1..locs.len()-1]}
    }
    // pub fn simplify(locs: &'a[FullLoc]) -> &'a[FullLoc] {
    //     let mut v = Vec::new();
    //     for i in 1..locs.len()-1 {
    //         if locs[i].
    //     }
    // }
}

pub struct Route<'a> {
    id:     RouteId,
    owner:  CompanyId,
    name:   &'a str,
    stops:  &'a[FullLoc],
    tracks: &'a[Track<'a>]
}

pub struct Train<'a> {
    id:    TrainId,
    owner: CompanyId,
    name:  &'a str,
    /// trains can only be assigned to one route at a time
    route: Weak<Route<'a>>,
    cars:  &'a[Weak<TrainCar<'a>>],
    spec:  SpecId,
}
pub struct TrainCar<'a> {
    id:    CarId,
    owner: CompanyId,
    /// cars can only be part of one train at a time
    train: Weak<Train<'a>>,
    /// how much cargo is in the car
    quant: u16,
    /// data about the car type
    spec:  Weak<CarSpec>,
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
