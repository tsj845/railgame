//! specifications for game systems data
//! eg. trains, cars, resources, etc.

use std::{borrow::Cow, sync::LazyLock};

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
pub struct TerrainCountReq {
    /// range terrain must be within (square of side length 1 + 2*range, centered on location)
    pub range: u8,
    #[serde(default)]
    /// min terrain of type within range
    pub min: u8,
    #[serde(default)]
    /// max terrain of type within range
    pub max: u8,
}
impl Default for TerrainCountReq {
    fn default() -> Self {
        Self { range: 0, min: 0, max: 255 }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum IndustryReq {
    Terrain{
        #[serde(rename = "type")]
        /// terrain type needed
        terrain: &'static str,
        #[serde(default)]
        counts: TerrainCountReq
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RateSpec {
    pub base: u16,
    pub scaling: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceRate {
    #[serde(rename = "type")]
    pub resource_type: &'static str,
    pub rate: RateSpec,
}

#[derive(Deserialize, Debug)]
/// industry spec
pub struct IndustrySpec {
    pub specid: SpecId,
    pub name: &'static str,
    pub requirements: Cow<'static, [IndustryReq]>,
    pub consumes: Cow<'static, [ResourceRate]>,
    pub produces: Cow<'static, [ResourceRate]>,
}

#[derive(Deserialize, Debug)]
/// buildable spec
pub struct BuildableSpec {}

#[derive(Deserialize, Debug, Clone)]
pub struct LocomotiveFuel {
    #[serde(rename = "type")]
    pub fuel_type: &'static str,
    /// appropriate units (eg. tons for coal, gal for water)
    pub capacity: u16,
    /// units per hour
    pub consumption: u16,
}
#[derive(Deserialize, Debug)]
pub struct LocomotiveSpec {
    pub specid: SpecId,
    pub name: &'static str,
    pub max_speed: u8,
    /// [in][frac] eg. 565 for 56"+1/2", 560 for 56"
    pub gauge: u16,
    // has_sand: bool,
    /// degrees
    pub min_curve: u8,
    pub weight: u32,
    pub fuel: Cow<'static, [LocomotiveFuel]>,
    /// lbf
    pub tractive_effort: u32,
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
    pub specid: SpecId,
    pub name: &'static str,
    /// TODO: determine if strings are a bad choice here (eg. replace with numeric ids)
    pub cargo_types: Cow<'static, [Cow<'static, str>]>,
    pub capacity: CarCapacity,
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

