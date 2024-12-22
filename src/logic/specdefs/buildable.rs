use super::super::specs::SpecId;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// buildable spec
pub struct BuildableSpec {
    pub id: SpecId,
    pub name: &'static str,
}
