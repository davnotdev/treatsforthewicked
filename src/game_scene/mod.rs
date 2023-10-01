use super::*;
use instant::Instant;
use std::time::Duration;

mod candyland;
mod intro;
mod load;
mod map;
mod outro;
mod score;

use load::*;
use score::{spawn_score_decrement, spawn_score_increment};

pub use map::Player;

#[derive(SingleResource, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Intro,
    Outro,
    Map,
    CandyLand,
}

pub fn run() {
    let galaxy = Galaxy::new();

    galaxy.insert_resource(GameState::single_resource(), GameState::Intro);

    window::window_run(
        galaxy,
        &[load_init, graphics::graphics_init, game_state_gate_init],
        &[
            graphics::graphics_update,
            game_quit,
            game_state_gate_update,
            score::score_render_update,
            score::score_increments_update,
        ],
    );
}

fn game_state_gate_init(galaxy: &Galaxy) {
    map::map_init(galaxy);
    candyland::candyland_init(galaxy);
}

fn game_state_gate_update(galaxy: &Galaxy) {
    if let Some(state) = galaxy.get_resource::<GameState, _>(GameState::single_resource()) {
        match *state {
            GameState::Intro => {
                drop(state);
                intro::intro_update(galaxy);
            }
            GameState::Outro => {
                drop(state);
                outro::outro_update(galaxy);
            }
            GameState::Map => {
                drop(state);
                map::map_update(galaxy);
            }
            GameState::CandyLand => {
                drop(state);
                candyland::candyland_update(galaxy);
            }
        }
    }
}

fn game_quit(galaxy: &Galaxy) {
    for ev in galaxy.get_events::<WindowEvent>() {
        if let WindowEventData::KeyboardInput { input, .. } = ev.0 {
            if input.virtual_keycode == Some(VirtualKeyCode::Q) {
                galaxy.set_exit();
            }
        }
    }
}
