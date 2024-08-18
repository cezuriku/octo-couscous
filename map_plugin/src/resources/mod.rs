use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    pub height: i32,
    pub width: i32,
    pub gold: i32,
}

#[derive(Resource)]
pub struct GoldTimer(pub Timer);
