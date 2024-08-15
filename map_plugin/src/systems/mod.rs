use std::cmp::min;

use crate::{components::*, resources::*};
use bevy::{
    prelude::*,
    render::view::window,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResized,
};

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

pub fn create_graphical_map(
    map: Res<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    cells: Query<(Entity, &Cell)>,
    window: Query<&Window>,
) {
    let window = window.single();
    let width: f32 = window.resolution.width() / map.width as f32;
    let height: f32 = window.resolution.height() / map.height as f32;
    let min_size: f32 = width.min(height);
    let radius = (min_size - 4.0) / 2.0;
    println!("Shape radius: {}", radius);

    for (entity, cell) in &cells {
        let shape = Mesh2dHandle(meshes.add(Circle { radius: radius }));
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(Color::hsl(200.0, 0.9, 0.7)),
            transform: Transform::from_xyz(
                (cell.x - 1) as f32 * radius * 2.0,
                (cell.y - 1) as f32 * radius * 2.0,
                0.0,
            ),
            ..default()
        });
    }
    commands.spawn(Camera2dBundle::default());
    println!("Windows size {}, {}", width, height);
}

pub fn update_graphical_map(
    map: Res<Map>,
    mut commands: Commands,
    cells: Query<(Entity, &Cell, &Transform)>,
    mut resize_event: EventReader<WindowResized>,
) {
    let mut event = resize_event.read().peekable();
    if event.peek().is_some() {
        let e = event.next().unwrap();
        let width = e.width / map.width as f32;
        let height = e.height / map.height as f32;
        let min_size: f32 = width.min(height);
        let radius = (min_size - 4.0) / 2.0;
        println!("Shape radius: {}", radius);

        for (entity, cell, _transform) in &cells {
            commands
                .entity(entity)
                .remove::<Transform>()
                .insert(Transform::from_xyz(
                    (cell.x as f32 + 0.5) * width - e.width / 2.0,
                    (cell.y as f32 + 0.5) * height - e.height / 2.0,
                    0.0,
                ));
        }
        println!("Windows size {}, {}", width, height);
    }
}
