use std::borrow::Cow;
use serde::Deserialize;

pub type ResourceId = u16;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RawResourceTypeConstraintSpec {
    Car{
        #[serde(rename = "taggedwith")]
        tagged_with:Cow<'static,[Cow<'static,str>]>
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct RawResourceTypeSpec {
    pub id: ResourceId,
    pub name: &'static str,
    pub desc: &'static str,
    #[serde(default)]
    pub constraints: Cow<'static, [RawResourceTypeConstraintSpec]>,
    #[serde(default)]
    pub implies: Cow<'static, [Cow<'static, str>]>,
}
impl Default for RawResourceTypeSpec {
    fn default() -> Self {
        Self {id:ResourceId::MAX,name:"INVALID",desc:"NO DESCRIPTION",constraints:Cow::Owned(Vec::new()),implies:Cow::Owned(Vec::new())}
    }
}

#[derive(Debug, Clone)]
pub enum ResourceTypeConstraintSpec {
    Car{tagged_with:Box<[ResourceId]>}
}
#[derive(Debug, Clone)]
pub struct ResourceTypeSpec {
    pub id: ResourceId,
    pub name: &'static str,
    /// short description
    pub desc: &'static str,
    pub constraints: Box<[ResourceTypeConstraintSpec]>,
    pub implies: Box<[ResourceId]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceSpec {
    pub id: ResourceId,
    pub name: &'static str,
}

#[allow(non_snake_case)]
pub fn _convert_RTSC(l: &Box<[RawResourceTypeSpec]>, it: &RawResourceTypeSpec) -> Box<[ResourceTypeConstraintSpec]> {
    let mut v = Vec::new();
    for i in 0..it.constraints.len() {
        let c = &it.constraints[i];
        match &c {
            &RawResourceTypeConstraintSpec::Car { tagged_with } => {
                v.push(ResourceTypeConstraintSpec::Car { tagged_with: _convert_RT_name_to_id(l, &tagged_with) });
            }
        };
    }
    return v.into_boxed_slice();
}
#[allow(non_snake_case)]
/// converts a resource name to an id
pub fn _convert_RT_name_to_id(l: &Box<[RawResourceTypeSpec]>, it: &Cow<'static, [Cow<'static, str>]>) -> Box<[ResourceId]> {
    let mut v = Vec::new();
    for i in 0..it.len() {
        for j in 0..l.len() {
            if l[j].name == it[i] {
                v.push(j as u16);
                break;
            }
        }
    }
    return v.into_boxed_slice();
}
#[allow(non_snake_case)]
/// converts from raw to resource type spec
pub fn _convert_RTS(l: Box<[RawResourceTypeSpec]>) -> Box<[ResourceTypeSpec]> {
    let mut v: Vec<ResourceTypeSpec> = Vec::with_capacity(l.len());
    for i in 0..l.len() {
        let it = &l[i];
        v.push(ResourceTypeSpec{id:it.id,name:it.name,desc:it.desc,constraints:_convert_RTSC(&l,it),implies:_convert_RT_name_to_id(&l,&it.implies)});
    }
    return v.into_boxed_slice();
}
