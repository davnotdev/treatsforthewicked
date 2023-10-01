use mewo::*;
use nalgebra_glm as glm;
use rand::prelude::*;

mod audio;
mod game_scene;
mod graphics;
mod window;

pub use audio::*;
pub use graphics::*;
pub use window::*;

fn main() {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    game_scene::run();
}
