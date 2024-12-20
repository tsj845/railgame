//! specifications for game systems data
//! eg. trains, cars, resources, etc.

use std::{borrow::Cow, sync::OnceLock};

use serde::{Serialize, Deserialize};

/// spec id, invalid/non-existant id represented by SpecId::MAX
pub type SpecId = u16;

type SpecSingleton<S> = OnceLock<Box<[S]>>;
/// spec singletons because it's just a way to keep the configs from disk in memory
static TRAIN_CAR_SPECS: SpecSingleton<CarSpec> = OnceLock::new();
static LOCOMOTIVE_SPECS: SpecSingleton<LocomotiveSpec> = OnceLock::new();
static BUILDABLE_SPECS: SpecSingleton<BuildableSpec> = OnceLock::new();

/// buildable industry spec
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
    StdFreight{weight:u16,volume:u16},
    Passenger{people:u16}
}
#[derive(Deserialize, Debug)]
pub struct CarSpec {
    specid: SpecId,
    name: &'static str,
    /// TODO: determine if strings are a bad choice here (eg. replace with numeric ids)
    cargo_types: Cow<'static, [Cow<'static, str>]>,
    capacity: CarCapacity,
}
#[allow(dead_code)]
fn init_car_specs() {
    if TRAIN_CAR_SPECS.get().is_some() {
        panic!("attempt to init multiple times");
    }
    let _ = TRAIN_CAR_SPECS.set(serde_json::from_str(include_str!("../../assets/objspecs/traincar.json")).unwrap());
}
#[allow(dead_code)]
fn init_loco_specs() {
    if LOCOMOTIVE_SPECS.get().is_some() {
        panic!("attempt to init multiple times");
    }
    let _ = LOCOMOTIVE_SPECS.set(serde_json::from_str(include_str!("../../assets/objspecs/locomotive.json")).unwrap());
}
pub fn get_car_spec(id: SpecId) -> &'static CarSpec {
    if let Some(specs) = TRAIN_CAR_SPECS.get() {
        return &specs[id as usize];
    } else {
        init_car_specs();
        return get_car_spec(id);
    }
}
pub fn get_loco_spec(id: SpecId) -> &'static LocomotiveSpec {
    if let Some(specs) = LOCOMOTIVE_SPECS.get() {
        return &specs[id as usize];
    } else {
        init_loco_specs();
        return get_loco_spec(id);
    }
}
pub fn get_car_specid(name: &'static str) -> SpecId {
    if let Some(specs) = TRAIN_CAR_SPECS.get() {
        for (i, spec) in specs.iter().enumerate() {
            if spec.name == name {
                return i as SpecId;
            }
        }
        return SpecId::MAX;
    } else {
        init_car_specs();
        return get_car_specid(name);
    }
}

