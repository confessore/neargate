use bevy::prelude::*;

use crate::{Cell, Unit};

pub mod game_state;

#[derive(Resource, Default)]
pub struct Game<'a> {
    pub board: Vec<Vec<Cell>>,
    pub camera_should_focus: Vec3,
    pub camera_is_focus: Vec3,
    pub unit: Unit<'a>,
}
