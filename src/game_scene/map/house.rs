use super::*;

const INTERACT_DISTANCE: f32 = 0.5;
const HOUSE_COOLDOWN: Duration = Duration::from_secs(30);

#[derive(CheapComponent, Clone, Copy)]
pub struct House {
    pub position: glm::Vec2,
    last_entered: Option<Instant>,
}

impl House {
    pub fn as_sprite(&self) -> Sprite {
        Sprite {
            visible: 1.0,
            texture_index: SPRITE_HOUSE,
            position: self.position,
            scale: glm::vec2(0.7, 0.7),
            color: glm::vec3(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }
}

pub fn house_init(galaxy: &Galaxy) {
    for y in -4..4 {
        for x in -4..4 {
            galaxy.insert_entity().insert(House {
                position: glm::vec2(x as f32 * 1.4, y as f32 * 1.7 + 1.0),
                last_entered: None,
            });
        }
    }
}

pub fn house_render_update(galaxy: &Galaxy) {
    for house in galaxy.query::<&House>().iter() {
        galaxy.insert_event(RendererDrawSprite(house.as_sprite()));
    }
}

pub fn house_interact_update(galaxy: &Galaxy) {
    let mut next_state = false;

    for ev in galaxy.get_events::<WindowEvent>() {
        if let WindowEventData::KeyboardInput { input, .. } = ev.0 {
            if input.virtual_keycode == Some(VirtualKeyCode::E)
                && input.state == ElementState::Released
            {
                let player = galaxy
                    .get_resource::<Player, _>(Player::single_resource())
                    .unwrap();

                for house in galaxy.query::<&mut House>().iter() {
                    if glm::distance(&player.position, &house.position) < INTERACT_DISTANCE {
                        if let Some(last_entered) = house.last_entered {
                            if Instant::now().duration_since(last_entered) < HOUSE_COOLDOWN {
                                audio_no(galaxy);
                                continue;
                            }
                        }
                        next_state = true;
                        house.last_entered = Some(Instant::now());
                    }
                }
            }
        }
    }

    if next_state {
        **galaxy
            .get_mut_resource::<GameState, _>(GameState::single_resource())
            .unwrap() = GameState::CandyLand;
    }
}
