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
    pub id:    ObjId,
    pub owner: Weak<Mutex<Company<'a>>>,
    pub start: FullLoc,
    pub end:   FullLoc,
    pub route: &'a[FullLoc]
}
// impl<'a> Track<'a> {
//     pub fn new(owner: Weak<Company<'a>>, start: FullLoc, end: FullLoc, route: &'a[FullLoc]) -> Self {
//         Self{id:get_next_trackid(),owner,start,end,route}
//     }
// }

pub struct Route<'a> {
    pub id:     ObjId,
    pub owner:  Weak<Mutex<Company<'a>>>,
    pub name:   &'a str,
    pub stops:  &'a[FullLoc],
    pub tracks: &'a[Weak<Mutex<Track<'a>>>]
}

pub struct Locomotive<'a> {
    pub id: ObjId,
    pub name: &'a str,
    pub spec: &'static LocomotiveSpec,
    pub owner: Weak<Mutex<Company<'a>>>,
    pub train: Weak<Mutex<Train<'a>>>,
}

pub struct Train<'a> {
    pub id:    ObjId,
    pub owner: Weak<Mutex<Company<'a>>>,
    pub name:  &'a str,
    /// trains can only be assigned to one route at a time
    pub route: Weak<Route<'a>>,
    /// one primary locomotive
    pub loco:  Weak<Mutex<Locomotive<'a>>>,
    pub helpers: &'a[Weak<Mutex<Locomotive<'a>>>],
    pub cars:  &'a[Weak<Mutex<TrainCar<'a>>>],
}
pub struct TrainCar<'a> {
    pub id:    ObjId,
    pub owner: Weak<Mutex<Company<'a>>>,
    /// cars can only be part of one train at a time
    pub train: Weak<Mutex<Train<'a>>>,
    /// how much cargo is in the car
    pub quant: u16,
    /// data about the car type
    pub spec:  &'static CarSpec,
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
