//! companies

use std::sync::{Arc,Mutex};

use super::{game::PlayerId, trains::{Locomotive, Route, Train, TrainCar}};

pub type CompanyId = u16;
/// 10k exact exist for each company
pub type Shares = u16;

/// measured in cents
pub type Money = i64;
/// measured in cents
#[allow(dead_code)]
pub struct Price(u64);
impl Price {
    pub fn from_money(amount: Money) -> Price {Price(match amount >= 0 {true=>amount as u64,_=>0})}
}

pub struct Company<'a> {
    pub id: CompanyId,
    pub name:   &'a str,
    pub ceo:    PlayerId,
    pub worth:  Money,
    pub routes: Vec<Arc<Mutex<Route<'a>>>>,
    pub locos:  Vec<Arc<Mutex<Locomotive<'a>>>>,
    pub trains: Vec<Arc<Mutex<Train<'a>>>>,
    pub cars:   Vec<Arc<Mutex<TrainCar<'a>>>>,
}

impl<'a> Company<'a> {
    pub fn gov() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            id: 0,
            name: "Government",
            ceo: 0,
            worth: 0,
            routes: Vec::new(),
            locos: Vec::new(),
            trains: Vec::new(),
            cars: Vec::new()
        }))
    }
}
