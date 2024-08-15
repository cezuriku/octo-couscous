use crate::systems::*;
use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::{Camera2dBundle, IntoSystemConfigs},
};

pub mod components;
pub mod events;
mod resources;
mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_map);
    }
}

pub struct GraphicalMapPlugin;

impl Plugin for GraphicalMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (create_map, create_graphical_map).chain());
        app.add_systems(Update, update_graphical_map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_map() {
        let map = components::BananaFactory { production: 5 };
        assert_eq!(map.production, 5)
    }
}
