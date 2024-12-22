//! all that has to do with tracks in world space

use super::{company::Company, specs::*};
use std::{rc::Weak, sync::{Arc, LazyLock, Mutex}};

use super::{company::CompanyId, world::FullLoc};

pub type ObjId = u32;

// static TRACK_ID_COUNTER: LazyLock<Arc<Mutex<ObjId>>> = LazyLock::new(||Arc::new(Mutex::new(0 as ObjId)));

// /// gets the next track id, threadsafe
// fn get_next_trackid() -> ObjId {
//     let mut l = TRACK_ID_COUNTER.lock().unwrap();
//     let r = *l;
//     *l += 1;
//     return r;
// }

/// use constructor function
pub struct Track<'a> {
    // id:    ObjId,
    owner: Weak<Company<'a>>,
    start: FullLoc,
    end:   FullLoc,
    route: &'a[FullLoc]
}
// impl<'a> Track<'a> {
//     pub fn new(owner: Weak<Company<'a>>, start: FullLoc, end: FullLoc, route: &'a[FullLoc]) -> Self {
//         Self{id:get_next_trackid(),owner,start,end,route}
//     }
// }

pub struct Route<'a> {
    // id:     ObjId,
    owner:  Weak<Company<'a>>,
    name:   &'a str,
    stops:  &'a[FullLoc],
    tracks: &'a[Track<'a>]
}

pub struct Locomotive<'a> {
    // id: ObjId,
    name: &'a str,
    spec: Weak<LocomotiveSpec>,
    owner: Weak<Company<'a>>,
    train: Weak<Train<'a>>,
}

pub struct Train<'a> {
    // id:    ObjId,
    owner: Weak<Company<'a>>,
    name:  &'a str,
    /// trains can only be assigned to one route at a time
    route: Weak<Route<'a>>,
    /// one primary locomotive
    loco:  Weak<Locomotive<'a>>,
    helpers: &'a[Weak<Locomotive<'a>>],
    cars:  &'a[Weak<TrainCar<'a>>],
}
pub struct TrainCar<'a> {
    // id:    ObjId,
    owner: Weak<Company<'a>>,
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
