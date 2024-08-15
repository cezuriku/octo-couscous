use bevy::prelude::Component;

#[derive(Debug, Copy, Clone, Component)]
pub struct BananaFactory {
    pub production: i32
}

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Empty,
    Wall,
    Factory
}

#[derive(Debug, Copy, Clone, Component)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub construction: Type,
}