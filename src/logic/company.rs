//! companies

use super::game::PlayerId;

pub type CompanyId = u16;
/// 10k exact exist for each company
pub type Shares = u16;

/// measured in cents
pub type Money = i64;

pub struct Company<'a> {
    id: CompanyId,
    name: &'a str,
    ceo: PlayerId,
    worth: Money,
}
