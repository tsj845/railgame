//! companies

use super::{game::PlayerId, trains::{Route, Train, TrainCar}};

pub type CompanyId = u16;
/// 10k exact exist for each company
pub type Shares = u16;

/// measured in cents
pub type Money = i64;
/// measured in cents
pub struct Price(u64);
impl Price {
    pub fn from_money(amount: Money) -> Price {Price(match amount >= 0 {true=>amount as u64,_=>0})}
}

pub struct Company<'a> {
    name:   &'a str,
    ceo:    PlayerId,
    worth:  Money,
    routes: Vec<Route<'a>>,
    trains: Vec<Train<'a>>,
    cars:   Vec<TrainCar<'a>>,
}
