//! tracks and manages all other logic structures
//! may be moved to src later

use super::{company::{Company, CompanyId, Money, Shares}, trains::Track, world::World};

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
    trains: Vec<()>
}
