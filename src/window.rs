use std::cell::UnsafeCell;

use minifb;

pub struct Window {
    window: UnsafeCell<minifb::Window>,
    frame_buffer: UnsafeCell<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        
        let window = match minifb::Window::new(
            title,
            width,
            height,
            minifb::WindowOptions::default(),
        ) {
            Ok(v) => v,
            Err(e) => panic!("Failed to create window, {e}")
        };

        let (width, height) = window.get_size();
        
        Self {
            window: UnsafeCell::new(window),
            frame_buffer: UnsafeCell::new(vec![0; width * height]),
            width: width,
            height: height,
        }
    }

    pub fn present(&self) {
        unsafe {
            match (*self.window.get()).update_with_buffer((*self.frame_buffer.get()).as_slice(), self.width, self.height) {
                Err(e) => println!("Failed to update framebuffer, {e}"),
                _ => {}
            }
        }
    }

    pub fn access_framebuffer(&self, accessing_function: impl FnOnce(&mut [u32])) {
        unsafe {
            accessing_function((*self.frame_buffer.get()).as_mut_slice());
        }
    }
}