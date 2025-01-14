use drm::control::Device as ControlDevice;
use drm::Device;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::{AsFd, BorrowedFd};

pub struct Card(File);

impl AsFd for Card {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Device for Card {}
impl ControlDevice for Card {}

impl Card {
    pub fn open(path: &str) -> Self {
        let mut options = OpenOptions::new();
        options.read(true).write(true);
        Card(options.open(path).unwrap())
    }
}