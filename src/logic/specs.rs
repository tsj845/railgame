//! specifications for game systems data
//! eg. trains, cars, resources, etc.

use std::{borrow::Cow, sync::{OnceLock,LazyLock}};

use serde::{Serialize, Deserialize};

macro_rules! specsingleton {
    ($file:expr) => {
        LazyLock::new(||serde_json::from_str($file).unwrap())
    };
}

/// spec id, invalid/non-existant id represented by SpecId::MAX
pub type SpecId = u16;
pub type ResourceId = u16;

type SpecSingleton<S> = LazyLock<Box<[S]>>;
/// spec singletons because it's just a way to keep the configs from disk in memory
static TRAIN_CAR_SPECS: SpecSingleton<CarSpec> = specsingleton!(include_str!("../../assets/objspecs/traincar.json"));
static LOCOMOTIVE_SPECS: SpecSingleton<LocomotiveSpec> = specsingleton!(include_str!("../../assets/objspecs/locomotive.json"));
static BUILDABLE_SPECS: SpecSingleton<BuildableSpec> = specsingleton!(include_str!("../../assets/objspecs/buildable.json"));
static INDUSTRY_SPECS: SpecSingleton<IndustrySpec> = specsingleton!(include_str!("../../assets/objspecs/industry.json"));

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum IndustryReq {
    Terrain(&'static str)
}

#[derive(Deserialize, Debug, Clone)]
pub struct RateSpec {
    base: u16,
    scaling: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceRate {
    #[serde(rename = "type")]
    resource_type: &'static str,
    rate: RateSpec,
}

#[derive(Deserialize, Debug)]
/// industry spec
pub struct IndustrySpec {
    specid: SpecId,
    name: &'static str,
    requirements: Cow<'static, [IndustryReq]>,
    consumes: Cow<'static, [ResourceRate]>,
    produces: Cow<'static, [ResourceRate]>,
}

#[derive(Deserialize, Debug)]
/// buildable spec
pub struct BuildableSpec {}

#[derive(Deserialize, Debug, Clone)]
pub struct LocomotiveFuel {
    #[serde(rename = "type")]
    fuel_type: &'static str,
    /// appropriate units (eg. tons for coal, gal for water)
    capacity: u16,
    /// units per hour
    consumption: u16,
}
#[derive(Deserialize, Debug)]
pub struct LocomotiveSpec {
    specid: SpecId,
    name: &'static str,
    max_speed: u8,
    /// [in][frac] eg. 565 for 56"+1/2", 560 for 56"
    gauge: u16,
    // has_sand: bool,
    /// degrees
    min_curve: u8,
    weight: u32,
    fuel: Cow<'static, [LocomotiveFuel]>,
    /// lbf
    tractive_effort: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CarCapacity {
    /// volume zero indicates no volume restriction
    StdFreight{weight:u16,volume:u16},
    Passenger{people:u16},
    Test(u16),
}
#[derive(Deserialize, Debug)]
pub struct CarSpec {
    specid: SpecId,
    name: &'static str,
    /// TODO: determine if strings are a bad choice here (eg. replace with numeric ids)
    cargo_types: Cow<'static, [Cow<'static, str>]>,
    capacity: CarCapacity,
}
pub fn get_car_spec(id: SpecId) -> &'static CarSpec {
    return &TRAIN_CAR_SPECS[id as usize];
}
pub fn get_loco_spec(id: SpecId) -> &'static LocomotiveSpec {
    return &LOCOMOTIVE_SPECS[id as usize];
}
pub fn get_indust_spec(id: SpecId) -> &'static IndustrySpec {
    return &INDUSTRY_SPECS[id as usize];
}
pub fn get_car_specid(name: &'static str) -> SpecId {
    for (i, spec) in TRAIN_CAR_SPECS.iter().enumerate() {
        if spec.name == name {
            return i as SpecId;
        }
    }
    return SpecId::MAX;
}

