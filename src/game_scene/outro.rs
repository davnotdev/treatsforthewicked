use super::*;

pub fn outro_update(galaxy: &Galaxy) {
    galaxy.insert_event(RendererDrawSprite(Sprite {
        visible: 1.0,
        texture_index: SPRITE_OUTRO,
        color: glm::vec3(1.0, 1.0, 1.0),
        rotation: 0.0,
        scale: glm::vec2(1.9, 1.9),
        position: glm::vec2(0.0, 0.0),
    }));
}
