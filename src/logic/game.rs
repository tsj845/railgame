//! tracks and manages all other logic structures
//! may be moved to src later

use std::{collections::LinkedList, sync::{Arc,Weak,Mutex}};

use super::{company::{Company, CompanyId, Money, Shares}, trains::{Track, Train}, world::World};
/// global mutable processing lock
/// aquire this THEN SET G_MUTPROC to false for any ops that can't have data changed while running
/// release this lock ONLY when G_MUTPROC is set back to true
pub static G_MUTPROC_LOCK: Mutex<()> = Mutex::new(());
pub static mut G_MUTPROC: bool = true;

pub type PlayerId = u16;

pub struct Player<'a> {
    pub id: PlayerId,
    pub name: &'a str,
    pub worth: Money,
    pub shares: Vec<(CompanyId, Shares)>,
}

pub struct Game<'a> {
    /// is this a networked game?
    pub netenabled: bool,
    /// is this player the host?
    pub ownssession: bool,
    pub world: World,
    pub companies: Vec<Arc<Mutex<Company<'a>>>>,
    pub players: Vec<Player<'a>>,
    pub tracks: Vec<Arc<Mutex<Track<'a>>>>,
    /// weak refs to all active trains
    /// LinkedList because trains will be added and removed randomly, prevents expensive moves
    pub trains: LinkedList<Weak<Mutex<Train<'a>>>>
}

impl<'a> Game<'a> {
    pub fn host(w: World, port: u16) -> Self {
        Self {
            netenabled: false,
            ownssession: true,
            world: w,
            companies: vec![Company::gov()],
            players: Vec::new(),
            tracks: Vec::new(),
            trains: LinkedList::new()
        }
    }
}
