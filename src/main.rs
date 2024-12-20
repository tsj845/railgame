use railgame::logic::specs::{get_car_spec, get_loco_spec, CarCapacity};

extern crate railgame;

fn main() {
    // println!("Hello, world!");
    println!("{:?}", get_car_spec(0));
    println!("{:?}", get_car_spec(1));
    println!("{:?}", get_loco_spec(0));
    // println!("{}", serde_json::to_string(&CarCapacity::Weight(15)).unwrap());
}
