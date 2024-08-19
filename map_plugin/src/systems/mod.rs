use crate::{components::*, resources::*};
use bevy::prelude::*;

pub fn create_map(mut commands: Commands) {
    let map = Map {
        width: 4,
        height: 4,
        gold: 200,
    };
    for x in 0..map.width {
        for y in 0..map.height {
            commands.spawn(Cell { x, y });
        }
    }
    commands.insert_resource(map);
    commands.insert_resource(GoldTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::components::GoldMine;
    use crate::resources::Map;
    use bevy::time::{TimePlugin, TimeUpdateStrategy};

    use super::*;

    #[test]
    fn gold_mine_generate_gold() {
        let mut app = App::new();
        let initial_gold = 13;
        let gold_production = 5;
        app.add_plugins(TimePlugin)
            .add_systems(Update, collect_gold)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                100,
            )));

        // Setup test entities
        app.world_mut().insert_resource(Map {
            gold: initial_gold,
            width: 5,
            height: 5,
        });
        app.world_mut()
            .insert_resource(GoldTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
        app.world_mut().spawn(GoldMine {
            production: gold_production,
        });

        // Between 0 and 0.5 seconds we should have only initial_gold
        for _ in 0..5 {
            app.update();
            let map = app.world().resource::<Map>();
            assert_eq!(
                map.gold, initial_gold,
                "We should now have {} gold",
                initial_gold
            );
        }

        // Between 0.5 and 1 second we should have harvested the first gold production
        for _ in 0..5 {
            app.update();
            let map = app.world().resource::<Map>();
            assert_eq!(
                map.gold,
                initial_gold + gold_production,
                "We should now have {} gold",
                initial_gold + gold_production
            );
        }

        // Between 0.5 and 1 second we should have harvested
        // the first and second gold productions
        for _ in 0..5 {
            app.update();
            let map = app.world().resource::<Map>();
            assert_eq!(
                map.gold,
                initial_gold + 2 * gold_production,
                "We should now have {} gold",
                initial_gold + 2 * gold_production
            );
        }
    }
}
