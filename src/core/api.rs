// The API is currently linux only, but
// in the future that will change.

use std::os::fd::{AsFd, BorrowedFd};

use crate::{other::types::{ColorFormat, Vec2}, platform::linux::card::Card};

use super::buffers::{Buffer, BufferStack};

/// Responsible for handling most of what toucan has to offer.
pub struct ToucanAPI {
    card: Card,
    _gbm_device: Option<gbm::Device<BorrowedFd<'static>>>,
    pub buffer_stack: BufferStack,
}

impl ToucanAPI {
    pub fn init(cardp: &str) -> Self {
        Self {
            card: Card::open(cardp),
            _gbm_device: None,
            buffer_stack: BufferStack::init()
        }
    }
    
    pub fn add_buffer(&mut self, size: Vec2<u32>, format: ColorFormat, depth: u32, bpp: u32) {
        self.buffer_stack.push(Buffer::new(
            &gbm::Device::new(self.card.as_fd()).unwrap(),
            &self.card,
            size,
            format,
            depth, bpp,
        ).expect("Failed to add buffer!"));
    }
}