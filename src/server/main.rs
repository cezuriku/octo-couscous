use bevy::prelude::*;

#[derive(Component)]
struct Factory {
    production: i32,
}

fn add_factory(mut commands: Commands) {
    commands.spawn(Factory { production: 5 });
}

#[derive(Resource)]
struct Room {
    bananas: i32,
}

#[derive(Resource)]
struct ProductionTimer(Timer);

fn print_bananas(time: Res<Time>, room: Res<Room>, mut timer: ResMut<ProductionTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Room bananas: {}", room.bananas)
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn collect_production(
    time: Res<Time>,
    mut room: ResMut<Room>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Factory>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for factory in &query {
            room.bananas += factory.production
        }
    }
}

pub struct OctoCouscousServerPlugin;

impl Plugin for OctoCouscousServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(ProductionTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .insert_resource(Room { bananas: 0 })
            .add_systems(Startup, add_factory)
            .add_systems(Update, (collect_production, print_bananas));
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, OctoCouscousServerPlugin))
        .run();
}
