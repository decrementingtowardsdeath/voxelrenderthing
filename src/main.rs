use app::App;

mod window;
mod renderer;
mod app;

fn main() {
    
    // init graphics
    // init other stuff
    let mut app = App::new();
    
    app.init();
    loop {
        app.update_loop();
    }
}
