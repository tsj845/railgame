use crate::logic::specs::get_res_specid;

use super::{super::specs::SpecId, resource::ResourceId};
use serde::Deserialize;
use std::borrow::Cow;

fn t() -> u8 {255u8}

#[derive(Deserialize, Debug, Clone)]
pub struct TerrainCountReq {
    /// range terrain must be within (square of side length 1 + 2*range, centered on location)
    pub range: u8,
    #[serde(default)]
    /// min terrain of type within range
    pub min: u8,
    #[serde(default = "t")]
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct RateSpec {
    pub base: u16,
    pub scaling: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawResourceRate {
    #[serde(rename = "type")]
    pub resource_type: &'static str,
    pub rate: RateSpec,
}

#[derive(Deserialize, Debug)]
/// industry spec
pub struct RawIndustrySpec {
    pub specid: SpecId,
    pub name: &'static str,
    pub requirements: Cow<'static, [IndustryReq]>,
    pub consumes: Cow<'static, [RawResourceRate]>,
    pub produces: Cow<'static, [RawResourceRate]>,
}

#[derive(Debug, Clone)]
pub struct ResourceRate {
    pub resource_type: ResourceId,
    pub rate: RateSpec,
}
#[derive(Debug, Clone)]
pub struct IndustrySpec {
    pub specid: SpecId,
    pub name: &'static str,
    pub requirements: Cow<'static, [IndustryReq]>,
    pub consumes: Box<[ResourceRate]>,
    pub produces: Box<[ResourceRate]>,
}
impl From<&RawIndustrySpec> for IndustrySpec {
    fn from(value: &RawIndustrySpec) -> Self {
        Self {
            specid: value.specid,
            name: value.name,
            requirements: value.requirements.clone(),
            consumes: _convert_RRR(&value.consumes),
            produces: _convert_RRR(&value.produces)
        }
    }
}

#[allow(non_snake_case)]
fn _convert_RRR(rr: &Cow<'static, [RawResourceRate]>) -> Box<[ResourceRate]> {
    let mut v = Vec::new();
    for i in 0..rr.len() {
        let it = &rr[i];
        v.push(ResourceRate{resource_type:get_res_specid(it.resource_type),rate:it.rate})
    }
    return v.into_boxed_slice();
}
#[allow(non_snake_case)]
pub fn _convert_RIS(isl: Box<[RawIndustrySpec]>) -> Box<[IndustrySpec]> {
    let l = isl.len();
    let mut v = Vec::with_capacity(l);
    for i in 0..l {
        let is = &isl[i];
        v.push(IndustrySpec::from(is));
    }
    return v.into_boxed_slice();
}
