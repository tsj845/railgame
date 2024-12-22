use super::super::specs::SpecId;
use serde::Deserialize;
use std::borrow::Cow;

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
