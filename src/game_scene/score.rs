use super::*;

const TOP_X: f32 = -0.8;
const TOP_Y: f32 = 0.8;
const OFFSET: f32 = 0.2;
const SCORE_INCREMENT_Y_INCREMENT: f32 = 0.02;
const WIN_SCORE: usize = 100;

#[derive(CheapComponent, Clone, Copy)]
pub struct ScorePop {
    position: glm::Vec2,
    is_positive: bool,
}

fn spawn(galaxy: &Galaxy, is_positive: bool) {
    let mut rng = rand::thread_rng();
    let x_rand = (rng.gen::<f32>() - 0.5) * 1.5;

    galaxy.insert_entity().insert(ScorePop {
        is_positive,
        position: glm::vec2(x_rand, if is_positive { -0.8 } else { 0.8 }),
    });
}

pub fn spawn_score_increment(galaxy: &Galaxy) {
    spawn(galaxy, true);
}

pub fn spawn_score_decrement(galaxy: &Galaxy) {
    spawn(galaxy, false);
}

pub fn score_render_update(galaxy: &Galaxy) {
    let player = galaxy
        .get_resource::<Player, _>(Player::single_resource())
        .unwrap();

    let state = galaxy
        .get_resource::<GameState, _>(GameState::single_resource())
        .unwrap();

    let mut digits = vec![];
    let mut n = player.score;
    while n > 0 {
        let digit = n % 10;
        digits.push(digit);
        n /= 10;
    }

    let delta = match *state {
        GameState::Map => player.position,
        GameState::Intro | GameState::Outro => glm::vec2(100.0, 100.0),
        _ => glm::vec2(0.0, 0.0),
    };

    let mut last_i = 0;
    for (i, digit) in digits.iter().rev().enumerate() {
        galaxy.insert_event(RendererDrawSprite(Sprite {
            visible: 1.0,
            texture_index: SPRITE_S0 + digit,
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            scale: glm::vec2(0.2, 0.2),
            position: glm::vec2(TOP_X + i as f32 * OFFSET, TOP_Y) + delta,
        }));
        last_i = i;
    }
    galaxy.insert_event(RendererDrawSprite(Sprite {
        visible: 1.0,
        texture_index: SPRITE_CANDY,
        color: glm::vec3(1.0, 1.0, 1.0),
        rotation: 0.0,
        scale: glm::vec2(0.2, 0.2),
        position: glm::vec2(TOP_X + (last_i + 1) as f32 * OFFSET, TOP_Y) + delta,
    }));

    if player.score >= WIN_SCORE {
        drop(state);
        **galaxy
            .get_mut_resource::<GameState, _>(GameState::single_resource())
            .unwrap() = GameState::Outro;
        audio_success(galaxy);
    }
}

pub fn score_increments_update(galaxy: &Galaxy) {
    let state = galaxy
        .get_resource::<GameState, _>(GameState::single_resource())
        .unwrap();

    for (e, score_inc) in galaxy.query::<&mut ScorePop>().eiter() {
        score_inc.position.y += if score_inc.is_positive {
            SCORE_INCREMENT_Y_INCREMENT
        } else {
            -SCORE_INCREMENT_Y_INCREMENT
        };
        if score_inc.position.y > 1.5 || score_inc.position.y < -1.5 {
            galaxy.remove_entity(e);
        }

        let delta = match *state {
            GameState::Map => {
                galaxy
                    .get_resource::<Player, _>(Player::single_resource())
                    .unwrap()
                    .position
            }
            GameState::Intro | GameState::Outro => glm::vec2(100.0, 100.0),
            _ => glm::vec2(0.0, 0.0),
        };

        galaxy.insert_event(RendererDrawSprite(Sprite {
            visible: 1.0,
            texture_index: if score_inc.is_positive {
                SPRITE_SPLUS
            } else {
                SPRITE_SMINUS
            },
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            scale: glm::vec2(0.2, 0.2),
            position: glm::vec2(score_inc.position.x, score_inc.position.y) + delta,
        }));
        galaxy.insert_event(RendererDrawSprite(Sprite {
            visible: 1.0,
            texture_index: SPRITE_CANDY,
            color: glm::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            scale: glm::vec2(0.2, 0.2),
            position: glm::vec2(score_inc.position.x + 0.2, score_inc.position.y) + delta,
        }));
    }
}
