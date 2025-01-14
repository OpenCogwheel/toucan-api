use toucan::types::*;

fn main() {
    let card = toucan::card::Card::open("/dev/dri/card1");
    let mut toucan_api = toucan::init(card);
    
    toucan_api.create_buffer(Vec2::new(800, 600), ColorFormat::Rgba8888, 32);
    
    toucan_api.create_image("output.png");
}
