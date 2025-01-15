// Currently not used for anything yet, as SDL is being used
// for early prototyping.

use crate::other::types::Vec2;

pub enum WindowState {
    Fullscreen,
    Borderless,
    Maximized,
    Minimized,
    Position(u32, u32),
}

pub struct WindowTraits {
    pub title: &'static str,
    pub size: Vec2<u32>,
    pub state: WindowState,
}

pub trait ToucanWindow {
    fn create(traits: WindowTraits) -> Self;
    fn poll_events(&mut self);
}