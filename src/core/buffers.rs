use std::os::fd::BorrowedFd;

use drm::control::{framebuffer::Handle, Device};
use gbm::{BufferObject, BufferObjectFlags, MappedBufferObject};

use crate::{
    other::types::{Vec2, ColorFormat},
    platform::linux::card::Card
};

pub struct Buffer {
    buffer_object: BufferObject<()>,
    framebuffer: Handle
}

impl<'a> Buffer {
    pub fn new(
        gbm_device: &gbm::Device<BorrowedFd<'a>>,
        card: &Card,
        size: Vec2<u32>,
        format: ColorFormat,
        depth: u32,
        bpp: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer_object = gbm_device
            .create_buffer_object::<()>(
                size.x, size.y,
                format.to_drm(),
                BufferObjectFlags::all()
            )?;
        
        let framebuffer = card.add_framebuffer(&buffer_object, depth, bpp)?;
        
        Ok(Self {
            buffer_object,
            framebuffer,
        })
    }
    
    pub fn map<T>(&self, mbfunc: &dyn Fn(&MappedBufferObject<()>) -> T) -> Result<T, Box<dyn std::error::Error>> {
        Ok(self.buffer_object.map(
            0, 0,
            self.buffer_object.width(), self.buffer_object.height(),
            mbfunc
        )?)
    }
    
    pub fn map_mut<T>(&mut self, mbfunc: &dyn Fn(&mut MappedBufferObject<()>) -> T) -> Result<T, Box<dyn std::error::Error>> {
        Ok(self.buffer_object.map_mut(
            0, 0,
            self.buffer_object.width(), self.buffer_object.height(),
            mbfunc
        )?)
    }
    
    pub fn framebuffer(&self) -> Handle { self.framebuffer }
}

pub struct BufferStack {
    buffers: Vec<Buffer>,
}

impl BufferStack {
    pub fn init() -> Self { Self { buffers: Vec::new() } }
    
    pub fn push(&mut self, buffer: Buffer) {
        self.buffers.push(buffer);
    }
    
    pub fn swap(&mut self) {
        let last_buffer = self.buffers.remove(0);
        self.buffers.push(last_buffer);
    }
    
    pub fn get_current_buffer(&mut self) -> &mut Buffer { &mut self.buffers[0] }
}