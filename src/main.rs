// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use game_plugin::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 1400.,
            height: 800.,
            title: "Bevy game".to_string(), // ToDo
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .insert_resource(bevy::winit::WinitSettings::game());

    app.run();
}
