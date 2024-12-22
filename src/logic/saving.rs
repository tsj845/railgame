//! support for saving the game
#![allow(non_camel_case_types)]
use super::game::{Game, G_MUTPROC, G_MUTPROC_LOCK};

pub struct SaveMeta {
    //
}

struct PS_Track {}
struct PS_Route {}
struct PS_TrainCar {}
struct PS_Locomotive {}
struct PS_Train {}
struct PS_Company {}
struct PS_World {}
struct PS_Game {
    world: PS_World,
}

pub struct RawSave {
    buf: Vec<u8>
}
impl RawSave {
    pub fn new(game: &mut Game) -> Result<Self, &'static str> {
        if !game.ownssession {
            return Err("not session owner");
        }
        let mut buf = Vec::new();
        // TODO: make sure there is a way to identify players in netenabled games
        // pause mutable processing
        let _guard = G_MUTPROC_LOCK.lock().unwrap();
        unsafe {G_MUTPROC = false;}
        // allow mutable processing to continue
        unsafe {G_MUTPROC = true;}
        return Ok(Self { buf });
    }
}
