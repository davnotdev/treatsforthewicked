use super::*;

const ABSOLUTE_STEAL_MAX: usize = 15;
const ANGER_DURATION: Duration = Duration::from_secs(3);
const ANGER_CHANCE: f64 = 0.2;

#[derive(SingleResource, Default, Clone, Copy)]
struct Candyland {
    candy_stolen: usize,
    time_of_anger: Option<Instant>,
}

impl Candyland {
    pub fn try_anger(&mut self, galaxy: &Galaxy) {
        let mut trigger_anger = self.candy_stolen >= ABSOLUTE_STEAL_MAX;

        let mut rng = rand::thread_rng();

        if rng.gen_bool(ANGER_CHANCE) {
            trigger_anger = true;
        }

        if trigger_anger && self.candy_stolen > 4 {
            audio_door(galaxy);
            self.time_of_anger = Some(Instant::now());
        }
    }

    pub fn reset(&mut self) {
        *self = Candyland::default();
    }
}

pub fn candyland_init(galaxy: &Galaxy) {
    galaxy.insert_resource(Candyland::single_resource(), Candyland::default());
}

pub fn candyland_update(galaxy: &Galaxy) {
    let mut camera = galaxy
        .get_mut_resource::<RendererCamera, _>(RendererCamera::single_resource())
        .unwrap();

    camera.position = glm::vec2(0.0, 0.0);

    galaxy.insert_event(RendererDrawSprite(Sprite {
        visible: 1.0,
        texture_index: SPRITE_HOUSE,
        color: glm::vec3(1.0, 1.0, 1.0),
        rotation: 0.0,
        scale: glm::vec2(1.0, 1.0),
        position: glm::vec2(0.0, 0.0),
    }));
    galaxy.insert_event(RendererDrawSprite(Sprite {
        visible: 1.0,
        texture_index: SPRITE_TAKEONE,
        color: glm::vec3(1.0, 1.0, 1.0),
        rotation: 0.0,
        scale: glm::vec2(0.4, 0.4),
        position: glm::vec2(-0.5, -0.7),
    }));
    galaxy.insert_event(RendererDrawSprite(Sprite {
        visible: 1.0,
        texture_index: SPRITE_BASKET,
        color: glm::vec3(1.0, 1.0, 1.0),
        rotation: 0.0,
        scale: glm::vec2(0.4, 0.4),
        position: glm::vec2(-0.5, -0.4),
    }));
    galaxy.insert_event(RendererDrawSprite(Sprite {
        visible: 1.0,
        texture_index: SPRITE_INPINFO,
        color: glm::vec3(1.0, 1.0, 1.0),
        rotation: 0.0,
        scale: glm::vec2(0.4, 0.4),
        position: glm::vec2(0.75, -0.4),
    }));

    let mut candyland = galaxy
        .get_mut_resource::<Candyland, _>(Candyland::single_resource())
        .unwrap();
    let mut player = galaxy
        .get_mut_resource::<Player, _>(Player::single_resource())
        .unwrap();

    for ev in galaxy.get_events::<WindowEvent>() {
        if let WindowEventData::KeyboardInput { input, .. } = ev.0 {
            if input.virtual_keycode == Some(VirtualKeyCode::Escape)
                && candyland.time_of_anger.is_none()
            {
                **galaxy
                    .get_mut_resource::<GameState, _>(GameState::single_resource())
                    .unwrap() = GameState::Map;
                candyland.reset();
            }
            if input.virtual_keycode == Some(VirtualKeyCode::E)
                && input.state == ElementState::Released
                && candyland.time_of_anger.is_none()
            {
                candyland.candy_stolen += 1;
                player.score += 1;
                spawn_score_increment(galaxy);
                audio_coin(galaxy);
                candyland.try_anger(galaxy);
            }
        }
    }

    if let Some(time_of_anger) = candyland.time_of_anger {
        galaxy.insert_event(RendererDrawSprite(Sprite {
            visible: 1.0,
            texture_index: SPRITE_GRANNY,
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            scale: glm::vec2(0.8, 0.8),
            position: glm::vec2(0.0, 0.0),
        }));

        if Instant::now().duration_since(time_of_anger) > ANGER_DURATION {
            **galaxy
                .get_mut_resource::<GameState, _>(GameState::single_resource())
                .unwrap() = GameState::Map;
            player.score = (player.score as isize - 6).clamp(0, 9999) as usize;
            audio_punch(galaxy);
            spawn_score_decrement(galaxy);
            candyland.reset();
        }
    }
}
