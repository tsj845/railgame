//! specifications for game systems data
//! eg. trains, cars, resources, etc.

use std::sync::LazyLock;

use super::specdefs::resource::_convert_RTS;
pub use super::specdefs::{resource::{ResourceId,ResourceSpec,ResourceTypeConstraintSpec,ResourceTypeSpec}, buildable::*, industry::*, locomotive::*, traincar::*};

macro_rules! specsingleton {
    ($file:expr) => {
        LazyLock::new(||serde_json::from_str($file).unwrap())
    };
    ($file:expr, $wrap:ident) => {
        LazyLock::new(||$wrap(serde_json::from_str($file).unwrap()))
    };
}

macro_rules! getfind_funcs {
    ($name:ident,$find:ident,$type:ty,$search:ident) => {
        pub fn $name (id: SpecId) -> $type {
            return &$search[id as usize];
        }
        pub fn $find (name: &'static str) -> SpecId {
            for (i, spec) in $search.iter().enumerate() {
                if spec.name == name {
                    return i as SpecId;
                }
            }
            return SpecId::MAX;
        }
    };
}

/// spec id, invalid/non-existant id represented by SpecId::MAX
pub type SpecId = u16;

type SpecSingleton<S> = LazyLock<Box<[S]>>;
static RESOURCE_TYPES: SpecSingleton<ResourceTypeSpec> = specsingleton!(include_str!("../../assets/objspecs/restype.json"), _convert_RTS);
static RESOURCE_SPECS: SpecSingleton<ResourceSpec> = specsingleton!(include_str!("../../assets/objspecs/resource.json"));
/// spec singletons because it's just a way to keep the configs from disk in memory
static TRAIN_CAR_SPECS: SpecSingleton<CarSpec> = specsingleton!(include_str!("../../assets/objspecs/traincar.json"));
static LOCOMOTIVE_SPECS: SpecSingleton<LocomotiveSpec> = specsingleton!(include_str!("../../assets/objspecs/locomotive.json"));
static BUILDABLE_SPECS: SpecSingleton<BuildableSpec> = specsingleton!(include_str!("../../assets/objspecs/buildable.json"));
static INDUSTRY_SPECS: SpecSingleton<IndustrySpec> = specsingleton!(include_str!("../../assets/objspecs/industry.json"));
// getfind_funcs!(get_res_spec,get_res_specid,&'static ResourceSpec, RESOURCE_SPECS);
getfind_funcs!(get_build_spec,get_build_specid,&'static BuildableSpec, BUILDABLE_SPECS);

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

pub struct ResourceType(ResourceId);
impl std::ops::Deref for ResourceType {
    type Target = ResourceTypeSpec;
    fn deref(&self) -> &Self::Target {
        return &RESOURCE_TYPES[self.0 as usize];
    }
}
pub struct Resource(ResourceId);
impl std::ops::Deref for Resource {
    type Target = ResourceSpec;
    fn deref(&self) -> &Self::Target {
        return &RESOURCE_SPECS[self.0 as usize];
    }
}
