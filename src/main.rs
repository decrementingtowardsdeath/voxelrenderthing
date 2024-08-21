#![allow(dead_code)]

use app::App;
use window::Window;

mod window;
mod renderer;
mod app;

fn main() {
    
    // init graphics
    // init other stuff
    let window = Window::new(640, 480, "Test Window");
    let mut app = App::new();
    
    app.init();
    loop {
        app.update_loop();
        window.present();
    }
}
