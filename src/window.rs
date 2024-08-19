use minifb;

pub struct Window {
    window: minifb::Window,
    frame_buffer: Vec<u32>,
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
            window: window,
            frame_buffer: vec![0; width * height],
            width: width,
            height: height,
        }
    }

    pub fn present(&mut self) {
        match self.window.update_with_buffer(&self.frame_buffer, self.width, self.height) {
            Err(e) => println!("Failed to update framebuffer, {e}"),
            _ => {}
        }
    }

    pub fn get_fb(&mut self) -> &mut [u32] {
        &mut self.frame_buffer
    }
}