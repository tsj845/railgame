//! tracks and manages all other logic structures
//! may be moved to src later

use std::{collections::LinkedList, rc::Weak};

use super::{company::{Company, CompanyId, Money, Shares}, trains::{Track, Train}, world::World};

pub type PlayerId = u16;

pub struct Player<'a> {
    id: PlayerId,
    name: &'a str,
    worth: Money,
    shares: Vec<(CompanyId, Shares)>,
}

pub struct Game<'a> {
    /// is this a networked game?
    netenabled: bool,
    /// is this player the host?
    ownssession: bool,
    world: World<'a>,
    companies: Vec<Company<'a>>,
    players: Vec<Player<'a>>,
    tracks: Vec<Track<'a>>,
    /// weak refs to all active trains
    /// LinkedList because trains will be added and removed randomly, prevents expensive moves
    trains: LinkedList<Weak<Train<'a>>>
}
