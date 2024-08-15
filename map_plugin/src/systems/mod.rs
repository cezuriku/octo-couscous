use crate::{components::*, resources::*};
use bevy::prelude::*;

pub fn create_map(mut commands: Commands) {
    let map = Map {
        width: 4,
        height: 4,
        gold: 0,
    };
    for x in 0..map.width {
        for y in 0..map.height {
            commands.spawn(Cell { x, y });
        }
    }
    commands.insert_resource(map);
    commands.insert_resource(GoldTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

pub fn collect_gold(
    time: Res<Time>,
    mut map: ResMut<Map>,
    mut timer: ResMut<GoldTimer>,
    query: Query<&GoldMine>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mine in &query {
            map.gold += mine.production
        }
    }
}
