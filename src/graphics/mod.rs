use super::*;
use mepeyew::*;

mod render;
mod texture;

pub use render::{
    graphics_init, graphics_update, RendererCamera, RendererDrawSprite, RendererInitLoadTextures,
};
pub use texture::TextureData;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Scene {
    view: glm::Mat4,
}

#[derive(Default, Clone, Copy)]
pub struct Sprite {
    pub visible: f32,
    pub texture_index: usize,
    pub color: glm::Vec3,
    pub rotation: f32,
    pub scale: glm::Vec2,
    pub position: glm::Vec2,
}

#[rustfmt::skip]
pub fn quad_vertices() -> &'static [f32] {
    &[
        -0.5,  0.5, 0.0, 0.0,
        -0.5, -0.5, 0.0, 1.0,
         0.5, -0.5, 1.0, 1.0,
         0.5,  0.5, 1.0, 0.0,
    ]
}

pub fn quad_indices() -> &'static [u32] {
    &[0, 1, 2, 0, 2, 3]
}
