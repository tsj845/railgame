use super::super::specs::SpecId;
use serde::Deserialize;
use std::borrow::Cow;

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
