use crate::{components::*, resources::*};
use bevy::prelude::*;

pub fn create_map(mut commands: Commands) {
    let map = Map {
        width: 4,
        height: 4,
    };
    for x in 0..map.width {
        for y in 0..map.height {
            commands.spawn(Cell {
                x: x,
                y: y,
                construction: Type::Empty,
            });
        }
    }
    commands.insert_resource(map);
}
