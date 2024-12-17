//! all that has to do with tracks in world space

use super::world::{Loc,FullLoc};

pub struct Track<'a> {
    start: FullLoc,
    end:   FullLoc,
    route: &'a[FullLoc]
}
impl<'a> Track<'a> {
    pub fn new(start: FullLoc, end: FullLoc, route: &'a[FullLoc]) -> Self {
        Self{start,end,route}
    }
}
