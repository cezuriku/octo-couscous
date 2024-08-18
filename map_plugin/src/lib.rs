use crate::systems::*;
use bevy::app::{App, Plugin, Startup, Update};

pub mod components;
pub mod events;
pub mod resources;
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

    use super::*;

    #[test]
    fn can_create_factory() {
        let factory = components::GoldMine { production: 5 };
        assert_eq!(factory.production, 5)
    }
}
