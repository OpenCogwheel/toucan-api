use toucan::other::types::*;

use sdl2::{event::Event, pixels::PixelFormatEnum, rect::Rect};

fn main() {
    let mut toucan_api = toucan::init("/dev/dri/card1");
    
    let sdl = sdl2::init().unwrap();
    let video_subsys = sdl.video().unwrap();
    let window = video_subsys
        .window("Test window", 800, 600)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    
    toucan_api.add_buffer(Vec2::new(800, 600), ColorFormat::Argb8888, 32, 32);
    toucan_api.add_buffer(Vec2::new(800, 600), ColorFormat::Argb8888, 32, 32);
    
    toucan_api.buffer_stack.get_current_buffer().map_mut(&mut |map_obj| {
        let buffer = map_obj.buffer_mut();
        for x in 0..800 {
            for y in 0..600 {
                buffer[y * 800 + x] = 255;
            }
        }
    }).unwrap();
    
    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        
        toucan_api.buffer_stack.swap();
        
        let buffer_data: Vec<u8> = toucan_api.buffer_stack.get_current_buffer().map(&move |mapped_buffer| {
            mapped_buffer.buffer().to_vec()
        }).unwrap();
        
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture_static(
            PixelFormatEnum::ARGB8888,
            800, 600
        ).unwrap();
        
        texture.update(None, &buffer_data, 800).unwrap();
        
        canvas.clear();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, 800, 600))).unwrap();
        canvas.present();
    }
}