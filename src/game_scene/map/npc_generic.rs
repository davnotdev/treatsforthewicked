use super::*;

const SPAWN_COUNT: usize = 12;
const WALK_SPEED: f32 = 0.01;
const BEATING_COOLDOWN: Duration = Duration::from_secs(12);
const HOUSE_IDLE_TIME: Duration = Duration::from_secs(2);
const PLAYER_PUNCH_RANGE: f32 = 0.6;
const CRYING_SLIDE_SPEED: f32 = 0.001;
const CRYING_TIME: Duration = Duration::from_secs(5);

#[derive(CheapComponent, Clone, Copy)]
pub struct NpcGeneric {
    position: glm::Vec2,
    sprite_texture: usize,
    last_beaten: Instant,
    state: NpcGenericState,
}

#[derive(Clone, Copy)]
enum NpcGenericState {
    Clueless,
    Walking(glm::Vec2),
    Idle(Instant),
    Crying(glm::Vec2, Instant),
}

impl NpcGeneric {
    pub fn as_sprite(&self) -> Sprite {
        Sprite {
            visible: 1.0,
            texture_index: match self.state {
                NpcGenericState::Crying(_, _) => SPRITE_CRYING,
                _ => self.sprite_texture,
            },
            position: self.position,
            scale: glm::vec2(0.35, 0.35),
            color: glm::vec3(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }

    pub fn can_be_beaten(&self) -> bool {
        Instant::now().duration_since(self.last_beaten) > BEATING_COOLDOWN
    }
}

pub fn npc_generic_init(galaxy: &Galaxy) {
    let mut rng = rand::thread_rng();
    for _ in 0..SPAWN_COUNT {
        let position = glm::vec2(
            (rng.gen::<f32>() - 0.5) * 0.75 * MAP_BOUND_SIZE,
            (rng.gen::<f32>() - 0.5) * 0.75 * MAP_BOUND_SIZE,
        );

        let sprite_texture = SPRITE_KID1 + rng.gen_range(0..2);

        galaxy.insert_entity().insert(NpcGeneric {
            position,
            sprite_texture,
            last_beaten: Instant::now(),
            state: NpcGenericState::Clueless,
        });
    }
}

pub fn npc_generic_update(galaxy: &Galaxy) {
    let mut rng = rand::thread_rng();
    let houses = galaxy.query::<&House>().iter().collect::<Vec<_>>();

    for npc in galaxy.query::<&mut NpcGeneric>().iter() {
        match npc.state {
            NpcGenericState::Clueless => {
                let goto_house_idx = rng.gen_range(0..houses.len());
                npc.state = NpcGenericState::Walking(houses[goto_house_idx].position)
            }
            NpcGenericState::Walking(goto_location) => {
                let dist = goto_location - npc.position;
                let dir = glm::normalize(&dist) * WALK_SPEED;
                npc.position += dir;

                if glm::length(&dist) < 0.3 {
                    npc.state = NpcGenericState::Idle(Instant::now())
                }
            }
            NpcGenericState::Idle(idle_time) => {
                if Instant::now().duration_since(idle_time) > HOUSE_IDLE_TIME {
                    npc.state = NpcGenericState::Clueless
                }
            }
            NpcGenericState::Crying(velocity, crying_time) => {
                npc.position += velocity;
                if Instant::now().duration_since(crying_time) > CRYING_TIME {
                    npc.state = NpcGenericState::Clueless
                }
            }
        };
    }
}

pub fn npc_generic_punch_update(galaxy: &Galaxy) {
    let mut player = galaxy
        .get_mut_resource::<Player, _>(Player::single_resource())
        .unwrap();

    for ev in galaxy.get_events::<WindowEvent>() {
        if let WindowEventData::KeyboardInput { input, .. } = ev.0 {
            if input.virtual_keycode == Some(VirtualKeyCode::F)
                && input.state == ElementState::Released
            {
                player.set_punch_frame();
                audio_punch(galaxy);
                for npc in galaxy.query::<&mut NpcGeneric>().iter() {
                    if glm::distance(&npc.position, &player.position) < PLAYER_PUNCH_RANGE
                        && npc.can_be_beaten()
                    {
                        let velocity = (player.position - npc.position) * CRYING_SLIDE_SPEED;
                        blow_candy(galaxy, npc.position);
                        npc.last_beaten = Instant::now();
                        npc.state = NpcGenericState::Crying(velocity, npc.last_beaten);
                    }
                }
            }
        }
    }
}

pub fn npc_generic_render_update(galaxy: &Galaxy) {
    for npc in galaxy.query::<&NpcGeneric>().iter() {
        galaxy.insert_event(RendererDrawSprite(npc.as_sprite()));
    }
}
