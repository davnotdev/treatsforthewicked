use super::*;

mod bounds;
mod candy;
mod house;
mod npc_bully;
mod npc_generic;
mod player;

use bounds::*;
use candy::*;
use house::*;
use npc_generic::*;
use player::*;

pub use player::Player;

pub fn map_init(galaxy: &Galaxy) {
    player_init(galaxy);
    npc_generic_init(galaxy);
    house_init(galaxy);
}

pub fn map_update(galaxy: &Galaxy) {
    house_interact_update(galaxy);
    house_render_update(galaxy);

    npc_generic_punch_update(galaxy);
    npc_generic_update(galaxy);
    npc_generic_render_update(galaxy);

    candy_movement_update(galaxy);
    candy_in_range_pickup_update(galaxy);
    candy_in_range_movement_update(galaxy);
    candy_render_update(galaxy);

    player_movement_update(galaxy);
    player_render_update(galaxy);
}
