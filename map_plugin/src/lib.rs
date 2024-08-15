use crate::systems::*;
use bevy::app::{App, Plugin, Startup, Update};

pub mod components;
pub mod events;
mod resources;
mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_map);
        app.add_systems(Update, collect_gold);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::time::{TimePlugin, TimeUpdateStrategy};
    use components::GoldMine;
    use resources::Map;

    use super::*;

    #[test]
    fn can_create_factory() {
        let factory = components::GoldMine { production: 5 };
        assert_eq!(factory.production, 5)
    }

    #[test]
    fn gold_mine_generate_gold() {
        let mut app = App::new();
        let gold_production = 3;
        app.add_plugins(TimePlugin)
            .add_systems(Startup, create_map)
            .add_systems(Update, collect_gold)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                100,
            )));

        // Setup test entities
        app.world_mut().spawn(GoldMine {
            production: gold_production,
        });

        // From 0 seconds to 0.5 we should have 0 gold.
        for _ in 0..5 {
            app.update();
            let map = app.world().resource::<Map>();
            assert_eq!(map.gold, 0, "No gold should be present at the beginning");
        }

        // After 0.5 seconds we should have 1 production
        for _ in 0..5 {
            app.update();
            let map = app.world().resource::<Map>();
            assert_eq!(
                map.gold, gold_production,
                "The first production should have produced {gold_production} gold"
            );
        }

        // After 0.5 seconds we should have 2 production
        for _ in 0..5 {
            app.update();
            let map = app.world().resource::<Map>();
            assert_eq!(
                map.gold,
                2 * gold_production,
                "We should now have {} gold",
                2 * gold_production
            );
        }
    }
}
