pub mod types;
pub mod card;
pub mod api;

use api::ToucanAPI;

pub use card::*;

pub fn init(card: Card) -> ToucanAPI {
    ToucanAPI::init(card)
}
