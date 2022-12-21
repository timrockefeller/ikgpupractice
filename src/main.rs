mod window;
mod render;
mod world;
use window::window_loop::run;
fn main() {
    pollster::block_on(run());
}
