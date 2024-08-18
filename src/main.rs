use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use map_plugin::MapPlugin;
use server_plugin::ServerPlugin;

const UPDATE_PER_SECOND: f64 = 20.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / UPDATE_PER_SECOND),
        )),
        MapPlugin,
        ServerPlugin,
    ))
    .run();
}
