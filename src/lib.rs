pub mod other;
pub mod core;
pub mod platform;

use core::api::ToucanAPI;

pub fn init(cardp: &str) -> ToucanAPI {
    ToucanAPI::init(cardp)
}
