use bevy::prelude::Component;

#[derive(Debug, Copy, Clone, Component)]

pub struct GoldMine {
    pub production: i32,
}

#[derive(Debug, Copy, Clone, Component)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}
