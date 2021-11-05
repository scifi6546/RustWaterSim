use crate::prelude::{Solver, WaterMarker};
use crate::GameState;
use bevy::prelude::*;
struct GameMenu;
pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(build_ui.system()));
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(run_ui.system()));
    }
}
/// Marks viscocoty change text button
struct ViscocityChange;
fn build_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("building ui");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                border: Rect::all(Val::Px(3.0)),
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.5, 0.5, 0.1).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Hello box",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text::with_section(
                    "Viscosity",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        border: Rect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexEnd,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
                            ..Default::default()
                        })
                        .with_children(|button| {
                            button.spawn_bundle(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    align_items: AlignItems::FlexStart,
                                    flex_direction: FlexDirection::Row,
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "<<",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(ViscocityChange);
                });
        })
        .insert(GameMenu);
}
fn run_ui(
    mut _commands: Commands,
    water_query: Query<&Box<dyn Solver>, With<WaterMarker>>,
    mut query: Query<&mut Text, With<ViscocityChange>>,
) {
    let viscosity = water_query
        .iter()
        .map(|_w| "todo: solver info")
        .next()
        .clone();
    if let Some(viscosity) = viscosity {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", viscosity);
        }
    }
}
