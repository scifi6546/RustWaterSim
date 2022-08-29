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
        .insert_resource(bevy::winit::WinitSettings {
            return_from_run: false,
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
        });

    app.run();
}
