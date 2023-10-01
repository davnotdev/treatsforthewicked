use super::*;

const FRICTION_FACTOR: f32 = 0.001;
const PICKUP_RANGE: f32 = 0.2;
const MOVEMENT_RANGE: f32 = 2.0;
const MOVEMENT_SCALAR: f32 = 0.0005;
const MAX_TRAVEL: f32 = 0.01;

#[derive(CheapComponent, Clone, Copy)]
pub struct Candy {
    position: glm::Vec2,
    velocity: glm::Vec2,
    travel: f32,
}

impl Candy {
    pub fn as_sprite(&self) -> Sprite {
        Sprite {
            visible: 1.0,
            texture_index: SPRITE_CANDY,
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            scale: glm::vec2(0.1, 0.1),
            position: self.position,
        }
    }
}

pub fn blow_candy(galaxy: &Galaxy, position: glm::Vec2) {
    let mut rng = rand::thread_rng();
    let candy_count: i32 = rng.gen_range(0..4);

    for _ in 0..candy_count {
        let direction_x = rng.gen::<f32>() - 0.5;
        let direction_y = rng.gen::<f32>() - 0.5;

        galaxy.insert_entity().insert(Candy {
            position,
            travel: 0.0,
            velocity: glm::vec2(direction_x, direction_y),
        });
    }
}

pub fn candy_movement_update(galaxy: &Galaxy) {
    for candy in galaxy.query::<&mut Candy>().iter() {
        if candy.travel < MAX_TRAVEL {
            let friction_factor = if candy.velocity.x.abs() < 0.01 {
                8.0
            } else {
                1.0
            };
            candy.velocity.x -= candy.velocity.x * FRICTION_FACTOR * friction_factor;

            let friction_factor = if candy.velocity.y.abs() < 0.01 {
                8.0
            } else {
                1.0
            };
            candy.velocity.y -= candy.velocity.y * FRICTION_FACTOR * friction_factor;

            candy.position.x += candy.velocity.x;
            candy.position.y += candy.velocity.y;

            candy.travel += glm::length(&candy.velocity);
        }
    }
}

pub fn candy_in_range_movement_update(galaxy: &Galaxy) {
    let player = galaxy
        .get_resource::<Player, _>(Player::single_resource())
        .unwrap();

    for candy in galaxy.query::<&mut Candy>().iter() {
        if glm::distance(&player.position, &candy.position) < MOVEMENT_RANGE {
            candy.position += glm::normalize(&(candy.position - player.position)) * MOVEMENT_SCALAR;
        }
    }
}

pub fn candy_in_range_pickup_update(galaxy: &Galaxy) {
    let mut player = galaxy
        .get_mut_resource::<Player, _>(Player::single_resource())
        .unwrap();

    for (e, candy) in galaxy.query::<&mut Candy>().eiter() {
        if glm::distance(&player.position, &candy.position) < PICKUP_RANGE {
            spawn_score_increment(galaxy);
            player.score += 1;
            audio_coin(galaxy);
            galaxy.remove_entity(e);
        }
    }
}

pub fn candy_render_update(galaxy: &Galaxy) {
    for candy in galaxy.query::<&Candy>().iter() {
        galaxy.insert_event(RendererDrawSprite(candy.as_sprite()));
    }
}
