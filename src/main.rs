use crate::app::App;
use winit::event_loop::EventLoop;

mod render;
mod app;
mod element;
mod views;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("run app error.");
}
