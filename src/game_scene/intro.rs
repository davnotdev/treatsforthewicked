use super::*;

pub fn intro_update(galaxy: &Galaxy) {
    galaxy.insert_event(RendererDrawSprite(Sprite {
        visible: 1.0,
        texture_index: SPRITE_INTRO,
        color: glm::vec3(1.0, 1.0, 1.0),
        rotation: 0.0,
        scale: glm::vec2(1.9, 1.9),
        position: glm::vec2(0.0, 0.0),
    }));
    for ev in galaxy.get_events::<WindowEvent>() {
        if let WindowEventData::KeyboardInput { .. } = ev.0 {
            **galaxy
                .get_mut_resource::<GameState, _>(GameState::single_resource())
                .unwrap() = GameState::Map;
        }
    }
}
