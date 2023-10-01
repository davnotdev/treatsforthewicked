use super::*;

const VELOCITY_INC: f32 = 0.0006;
const FRICTION_FACTOR: f32 = 0.001;
const MAX_COMPONENT_SPEED: f32 = 0.02;
const PUNCH_FRAME_DURATION: Duration = Duration::from_millis(400);

#[derive(SingleResource, Default, Clone, Copy)]
pub struct Player {
    pub score: usize,
    velocity: glm::Vec2,
    pub position: glm::Vec2,
    state: PlayerState,
}

#[derive(Default, Clone, Copy)]
enum PlayerState {
    #[default]
    Normal,
    Punching(Instant),
}

impl Player {
    pub fn as_sprite(&self) -> Sprite {
        Sprite {
            visible: 1.0,
            texture_index: match self.state {
                PlayerState::Normal => SPRITE_OURKID,
                PlayerState::Punching(_) => SPRITE_KIDPUNCH,
            },
            position: self.position,
            scale: glm::vec2(0.3, 0.3),
            color: glm::vec3(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }

    pub fn set_punch_frame(&mut self) {
        self.state = PlayerState::Punching(Instant::now())
    }
}

pub fn player_init(galaxy: &Galaxy) {
    galaxy.insert_resource(Player::single_resource(), Player::default());
}

pub fn player_movement_update(galaxy: &Galaxy) {
    let mut player = galaxy
        .get_mut_resource::<Player, _>(Player::single_resource())
        .unwrap();

    let mut movement_key_pressed = false;

    for ev in galaxy.get_events::<WindowEvent>() {
        if let WindowEventData::KeyboardInput { input, .. } = ev.0 {
            if let Some(input) = input.virtual_keycode {
                match input {
                    VirtualKeyCode::W => {
                        player.velocity.y += VELOCITY_INC;
                        movement_key_pressed = true;
                    }
                    VirtualKeyCode::A => {
                        player.velocity.x -= VELOCITY_INC;
                        movement_key_pressed = true;
                    }
                    VirtualKeyCode::S => {
                        player.velocity.y -= VELOCITY_INC;
                        movement_key_pressed = true;
                    }
                    VirtualKeyCode::D => {
                        player.velocity.x += VELOCITY_INC;
                        movement_key_pressed = true;
                    }
                    _ => (),
                }
            }
        }
    }

    player.velocity.x = player
        .velocity
        .x
        .clamp(-MAX_COMPONENT_SPEED, MAX_COMPONENT_SPEED);
    player.velocity.y = player
        .velocity
        .y
        .clamp(-MAX_COMPONENT_SPEED, MAX_COMPONENT_SPEED);

    let friction_factor = if player.velocity.x.abs() < 0.01 || !movement_key_pressed {
        10.0
    } else {
        1.0
    };
    player.velocity.x -= player.velocity.x * FRICTION_FACTOR * friction_factor;

    let friction_factor = if player.velocity.y.abs() < 0.01 || !movement_key_pressed {
        10.0
    } else {
        1.0
    };
    player.velocity.y -= player.velocity.y * FRICTION_FACTOR * friction_factor;

    player.position.x += player.velocity.x;
    player.position.y += player.velocity.y;
}

pub fn player_render_update(galaxy: &Galaxy) {
    let mut player = galaxy
        .get_mut_resource::<Player, _>(Player::single_resource())
        .unwrap();

    if let PlayerState::Punching(punch_time) = player.state {
        if Instant::now().duration_since(punch_time) > PUNCH_FRAME_DURATION {
            player.state = PlayerState::Normal;
        }
    }

    galaxy.insert_event(RendererDrawSprite(player.as_sprite()));

    let mut camera = galaxy
        .get_mut_resource::<RendererCamera, _>(RendererCamera::single_resource())
        .unwrap();
    camera.position = player.position;
}
