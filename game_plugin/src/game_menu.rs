use crate::prelude::{SolveInfo, Solver, WaterMarker};
use crate::GameState;
use bevy::prelude::*;
struct GameMenu;
pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterial>();
        app.init_resource::<GuiState>();
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(build_ui.system()));
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(run_ui.system()));

        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(show_velocity_button.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(solve_info.system()),
        );
    }
}
/// Marks viscocoty change text button
struct ViscocityChange;
struct SolveInfoLabel;
struct ButtonMaterial {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}
impl FromWorld for ButtonMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.35, 0.35).into()),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuiState {
    pub show_velocities: bool,
    pub show_water: bool,
}
impl Default for GuiState {
    fn default() -> Self {
        Self {
            show_velocities: false,
            show_water: true,
        }
    }
}
/// Marks Show Velocities button
struct ShowVelocities;
/// Marks show water
struct ShowWater;
fn build_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_material: Res<ButtonMaterial>,
) {
    println!("building ui");
    let gui_state = GuiState::default();

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
                .insert(SolveInfoLabel);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_material.normal.clone(),
                    ..Default::default()
                })
                .insert(ShowVelocities)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Show Velocities",
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
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: match gui_state.show_water {
                        true => button_material.pressed.clone(),
                        false => button_material.normal.clone(),
                    },
                    ..Default::default()
                })
                .insert(ShowWater)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Show Water",
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
        })
        .insert(GameMenu);
    commands.spawn().insert(gui_state);
}
fn show_velocity_button(
    button_materials: Res<ButtonMaterial>,
    mut gui_state_query: Query<&mut GuiState, ()>,
    mut queries: QuerySet<(
        Query<
            (&Interaction, &mut Handle<ColorMaterial>, &Children),
            (Changed<Interaction>, With<ShowVelocities>),
        >,
        Query<
            (&Interaction, &mut Handle<ColorMaterial>, &Children),
            (Changed<Interaction>, With<ShowWater>),
        >,
    )>,
) {
    let mut gui_state = gui_state_query.iter_mut().next();
    if gui_state.is_none() {
        error!("gui state not found");
        return;
    }
    let mut gui_state = gui_state.unwrap();
    for (interation, mut material, children) in queries.q0_mut().iter_mut() {
        match *interation {
            Interaction::Clicked => {
                gui_state.show_velocities = !gui_state.show_velocities;
                if gui_state.show_velocities {
                    *material = button_materials.pressed.clone();
                } else {
                    *material = button_materials.normal.clone();
                }
            }
            Interaction::Hovered => {
                if !gui_state.show_velocities {
                    *material = button_materials.hovered.clone();
                }
            }
            Interaction::None => {
                if gui_state.show_velocities {
                    *material = button_materials.pressed.clone();
                }
            }
        }
    }
    for (interation, mut material, children) in queries.q1_mut().iter_mut() {
        match *interation {
            Interaction::Clicked => {
                gui_state.show_water = !gui_state.show_water;
                if gui_state.show_water {
                    *material = button_materials.pressed.clone();
                } else {
                    *material = button_materials.normal.clone();
                }
            }
            Interaction::Hovered => {
                if !gui_state.show_water {
                    *material = button_materials.hovered.clone();
                }
            }
            Interaction::None => {
                if gui_state.show_water {
                    *material = button_materials.pressed.clone();
                }
            }
        }
    }
}
fn solve_info(
    mut _commands: Commands,
    solve_query: Query<&Vec<SolveInfo>, With<WaterMarker>>,
    mut query: Query<&mut Text, With<SolveInfoLabel>>,
) {
    if let Some(info_vec) = solve_query.iter().next() {
        let formatted_str = info_vec
            .iter()
            .map(|info| format!("{}: {}", info.name, info.data))
            .fold(String::new(), |acc, x| acc + "\n" + &x);
        for mut text in query.iter_mut() {
            text.sections[0].value = formatted_str.clone();
        }
    }
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
