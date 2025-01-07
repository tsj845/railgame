use super::super::specs::SpecId;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

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
