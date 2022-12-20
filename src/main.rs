mod window;
mod render;
use window::window_loop::run;
fn main() {
    pollster::block_on(run());
}
