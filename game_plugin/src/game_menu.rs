use crate::prelude::{FiniteSolver, SolveInfo, WaterMarker};
use crate::GameState;
use bevy::prelude::*;
use std::cmp::{max, min};
struct GameMenu;
pub struct GameMenuPlugin;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum GuiLabel {
    SidePanel,
    BottomPanel,
}
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterial>();
        app.init_resource::<GuiState>();
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(ui.system())
                .label(GuiLabel::SidePanel),
        );
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(run_ui.system()));

        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(show_velocity_button.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(play_button.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(pause_button.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(solve_info.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(show_speed.system()));
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
enum SpeedDirection {
    Increasing,
    Decreasing,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuiState {
    pub show_velocities: bool,
    pub show_water: bool,
    pub water_speed: u32,
    /// whether or not to increase speed when play button is clicked
    speed_direction: SpeedDirection,
}
impl Default for GuiState {
    fn default() -> Self {
        Self {
            show_velocities: false,
            show_water: true,
            water_speed: 0,
            speed_direction: SpeedDirection::Increasing,
        }
    }
}
const MAX_WATER_SPEED: u32 = 16;
/// Marker for play button
struct PlayButton;
struct PauseButton;
struct PauseTexture;
struct PlayTexture;
struct ShowSpeed;
/// Marks Show Velocities button
struct ShowVelocities;
/// Marks show water
struct ShowWater;
fn ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_material: Res<ButtonMaterial>,
) {
    let gui_state = GuiState::default();
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexEnd,
                        align_self: AlignSelf::Stretch,
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
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        border: Rect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        align_self: AlignSelf::FlexStart,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
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
                        .insert(PauseButton)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(50.0), Val::Auto),
                                        ..Default::default()
                                    },
                                    material: materials
                                        .add(asset_server.load("textures/pause.png").into()),
                                    ..Default::default()
                                })
                                .insert(PauseTexture)
                                .insert(Interaction::default());
                        });
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
                        .insert(PlayButton)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(50.0), Val::Auto),
                                        ..Default::default()
                                    },
                                    material: materials
                                        .add(asset_server.load("textures/play.png").into()),
                                    ..Default::default()
                                })
                                .insert(PlayTexture)
                                .insert(Interaction::default());
                        });

                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                format!("x 1"),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(ShowSpeed);
                });
        });
    commands.spawn().insert(gui_state);
}
fn show_speed(
    gui_state_query: Query<&GuiState, Changed<GuiState>>,
    mut query: Query<&mut Text, With<ShowSpeed>>,
) {
    let gui_state = if let Some(state) = gui_state_query.iter().next() {
        state
    } else {
        return;
    };
    for mut text in query.iter_mut() {
        if gui_state.water_speed == 0 {
            text.sections[0].value = "Paused".to_string();
        } else {
            text.sections[0].value = format!("{}x", gui_state.water_speed);
        }
    }
}
fn pause_button(
    button_materials: Res<ButtonMaterial>,
    mut gui_state_query: Query<&mut GuiState, ()>,
    mut queries: QuerySet<(
        Query<
            (&Interaction),
            (
                With<Interaction>,
                Or<(With<PauseButton>, With<PauseTexture>)>,
            ),
        >,
        Query<&mut Handle<ColorMaterial>, With<PauseButton>>,
    )>,
) {
    let mut gui_state = if let Some(state) = gui_state_query.iter_mut().next() {
        state
    } else {
        return;
    };
    let interaction = queries
        .q0()
        .iter()
        .fold(Interaction::None, |acc, x| match acc {
            Interaction::Clicked => acc,
            Interaction::Hovered => match x {
                Interaction::Clicked => Interaction::Clicked,
                Interaction::Hovered => Interaction::Hovered,
                Interaction::None => Interaction::Hovered,
            },
            Interaction::None => match x {
                Interaction::Clicked => Interaction::Clicked,
                Interaction::Hovered => Interaction::Hovered,
                Interaction::None => Interaction::None,
            },
        });
    let mut button_mat = if let Some(mat) = queries.q1_mut().iter_mut().next() {
        mat
    } else {
        return;
    };
    if interaction != Interaction::None {
        info!("{:?}", interaction);
    }
    match interaction {
        Interaction::Clicked => {
            *button_mat = button_materials.pressed.clone();
            if gui_state.water_speed >= 1 {
                gui_state.water_speed = 0;
            } else {
                gui_state.water_speed = 1;
            }
        }
        Interaction::Hovered => {
            *button_mat = button_materials.hovered.clone();
        }
        Interaction::None => {
            if gui_state.water_speed >= 1 {
                *button_mat = button_materials.normal.clone();
            } else {
                *button_mat = button_materials.pressed.clone();
            }
        }
    };
}
fn play_button(
    button_materials: Res<ButtonMaterial>,
    mut gui_state_query: Query<&mut GuiState, ()>,
    mut queries: QuerySet<(
        Query<
            (&Interaction),
            (
                Changed<Interaction>,
                Or<(With<PlayButton>, With<PlayTexture>)>,
            ),
        >,
        Query<&mut Handle<ColorMaterial>, With<PlayButton>>,
    )>,
) {
    let interaction = queries.q0().iter().next().copied();
    let mut gui_state = if let Some(state) = gui_state_query.iter_mut().next() {
        state
    } else {
        return;
    };
    let mut play_material = if let Some(mat) = queries.q1_mut().iter_mut().next() {
        mat
    } else {
        return;
    };
    if gui_state.water_speed == 0 {
        *play_material = button_materials.normal.clone();
    }

    if let Some(interaction) = interaction {
        info!("play button {:?}", interaction);
        match interaction {
            Interaction::Clicked => {
                let mut new_speed;
                match gui_state.speed_direction {
                    SpeedDirection::Increasing => {
                        new_speed = max(gui_state.water_speed * 2, 1);
                        if new_speed > MAX_WATER_SPEED {
                            gui_state.speed_direction = SpeedDirection::Decreasing;
                            new_speed = gui_state.water_speed / 2;
                        }
                    }
                    SpeedDirection::Decreasing => {
                        if gui_state.water_speed != 1 && gui_state.water_speed != 0 {
                            new_speed = max(gui_state.water_speed / 2, 1);
                        } else {
                            gui_state.speed_direction = SpeedDirection::Increasing;
                            new_speed = gui_state.water_speed * 2;
                        }
                    }
                };
                info!(
                    "new speed: {}, old speed: {}",
                    new_speed, gui_state.water_speed
                );
                gui_state.water_speed = new_speed;
                *play_material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *play_material = button_materials.hovered.clone();
            }
            Interaction::None => {}
        }
    }
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
    water_query: Query<&FiniteSolver, With<WaterMarker>>,
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
