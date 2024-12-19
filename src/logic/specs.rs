//! specifications for game systems data
//! eg. trains, cars, resources, etc.

use std::cell::OnceCell;

pub type SpecId = ();

/// singleton spec because it's just a way to keep the configs from disk in memory
static mut SPECS: OnceCell<()> = OnceCell::new();

struct Spec {
    //
}
