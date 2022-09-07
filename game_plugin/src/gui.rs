use crate::prelude::{
    aabb_barrier_from_transform, build_barrier, despawn_gui, AABBMaterial, AddBoxButton,
    FontAssets, GameEntity, GameMenu, GameState, GuiState, LeaveButton, LeaveText, PauseButton,
    PauseTexture, PlayButton, PlayTexture, SaveWaterButton, ShowSpeed, ShowVelocities, ShowWater,
    SolveInfoLabel, SolveInfoVec, SpeedDirection, ViscocityChange, WaterMarker, GUI_STYLE,
    MAX_WATER_SPEED, WATER_SIZE,
};
use bevy::prelude::*;
use nalgebra::Vector2;
use std::cmp::max;
use water_sim::{PreferredSolver, Solver};
#[derive(Clone, Debug, Copy)]
pub struct GuiRunner {
    pub active_state: GameState,
}
impl Plugin for GuiRunner {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuiState>();

        app.add_system_set(SystemSet::on_enter(self.active_state).with_system(init));

        app.add_system_set(
            SystemSet::on_update(self.active_state)
                .with_system(show_velocity_button)
                .with_system(run_ui),
        )
        .add_system_set(SystemSet::on_update(self.active_state).with_system(play_button))
        .add_system_set(SystemSet::on_update(self.active_state).with_system(pause_button));
        app.add_system_set(SystemSet::on_update(self.active_state).with_system(solve_info))
            .add_system_set(
                SystemSet::on_update(self.active_state)
                    .with_system(show_speed)
                    .with_system(solve_info)
                    .with_system(leave_button)
                    .with_system(save_water)
                    .with_system(add_box_button),
            )
            .add_system_set(
                SystemSet::on_exit(self.active_state)
                    .with_system(despawn_gui)
                    .with_system(cleanup_ui),
            );
    }
    fn name(&self) -> &str {
        "Gui Runner"
    }
}
pub fn init(mut commands: Commands) {
    commands.insert_resource(GuiState::default())
}
/// constructs time bar at bottom of game
pub fn build_play_menu(
    parent: &mut ChildBuilder<'_, '_, '_>,
    asset_server: &Res<AssetServer>,
    gui_state: &GuiState,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_grow: 2.0,
                align_self: AlignSelf::Stretch,
                align_items: AlignItems::Stretch,
                align_content: AlignContent::Stretch,

                ..Default::default()
            },
            color: UiColor(Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            }),
            ..Default::default()
        })
        .insert(GameEntity)
        .with_children(|parent| {
            build_sidebar(parent, asset_server, &gui_state);
            build_playbar(parent, asset_server, &gui_state);
        })
        .insert(GameEntity);
}
pub fn build_sidebar(
    parent: &mut ChildBuilder<'_, '_, '_>,
    asset_server: &Res<AssetServer>,
    gui_state: &GuiState,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                border: UiRect::all(Val::Px(3.0)),
                flex_grow: 2.0,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexEnd,
                align_self: AlignSelf::Stretch,
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                ..Default::default()
            },
            color: UiColor(GUI_STYLE.side_panel_color),
            ..Default::default()
        })
        .insert(GameEntity)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: GUI_STYLE.text_color,
                        },
                    ),
                    ..Default::default()
                })
                .insert(SolveInfoLabel)
                .insert(GameEntity);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(GUI_STYLE.button_normal_color),
                    ..Default::default()
                })
                .insert(GameEntity)
                .insert(ShowVelocities)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::from_section(
                                "Show Velocities",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: GUI_STYLE.button_text_color.clone(),
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(GameEntity);
                })
                .insert(GameEntity);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(match gui_state.show_water {
                        true => GUI_STYLE.button_pressed_color,
                        false => GUI_STYLE.button_normal_color,
                    }),
                    ..Default::default()
                })
                .insert(ShowWater)
                .insert(GameEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::from_section(
                                "Show Water",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: GUI_STYLE.button_text_color,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(GameEntity);
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(GUI_STYLE.button_normal_color),
                    ..Default::default()
                })
                .insert(GameEntity)
                .insert(AddBoxButton)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::from_section(
                                "Add Box",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: GUI_STYLE.button_text_color,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(GameEntity);
                });
            #[cfg(feature = "native")]
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_material.normal.clone(),
                    ..Default::default()
                })
                .insert(GameEntity)
                .insert(SaveWaterButton)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Export Water Heights",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: GUI_STYLE.button_text_color,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(GameEntity);
                });
        })
        .insert(GameMenu)
        .insert(GameEntity);
}
pub fn build_playbar(
    parent: &mut ChildBuilder<'_, '_, '_>,
    asset_server: &Res<AssetServer>,
    gui_state: &GuiState,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            color: UiColor(GUI_STYLE.bottom_panel_color),
            ..Default::default()
        })
        .insert(GameEntity)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        align_self: AlignSelf::FlexStart,
                        ..Default::default()
                    },
                    color: UiColor(GUI_STYLE.bottom_subpanel_color),
                    ..Default::default()
                })
                .insert(GameEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: UiColor(GUI_STYLE.button_normal_color),

                            ..Default::default()
                        })
                        .insert(PauseButton)
                        .insert(GameEntity)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(50.0), Val::Auto),
                                        ..Default::default()
                                    },
                                    image: UiImage(asset_server.load("textures/pause.png")),
                                    ..Default::default()
                                })
                                .insert(PauseTexture)
                                .insert(GameEntity)
                                .insert(Interaction::default());
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: UiColor(GUI_STYLE.button_normal_color),
                            ..Default::default()
                        })
                        .insert(PlayButton)
                        .insert(GameEntity)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(50.0), Val::Auto),
                                        ..Default::default()
                                    },
                                    image: UiImage(asset_server.load("textures/play.png")),
                                    ..Default::default()
                                })
                                .insert(PlayTexture)
                                .insert(GameEntity)
                                .insert(Interaction::default());
                        });
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::from_section(
                                "x 1",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: GUI_STYLE.text_color,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(GameEntity)
                        .insert(ShowSpeed);
                })
                .insert(GameEntity);

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(GUI_STYLE.button_normal_color),
                    ..Default::default()
                })
                .insert(LeaveButton)
                .insert(GameEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::from_section(
                                "Exit".to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: GUI_STYLE.button_text_color,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(LeaveText)
                        .insert(GameEntity);
                });
        })
        .insert(GameEntity);
}
fn show_speed(mut gui_state: ResMut<GuiState>, mut query: Query<&mut Text, With<ShowSpeed>>) {
    for mut text in query.iter_mut() {
        if gui_state.water_speed == 0 {
            text.sections[0].value = "Paused".to_string();
        } else {
            text.sections[0].value = format!("{}x", gui_state.water_speed);
        }
    }
}

fn pause_button(
    mut gui_state: ResMut<GuiState>,
    mut queries: ParamSet<(
        Query<
            &Interaction,
            (
                With<Interaction>,
                Or<(With<PauseButton>, With<PauseTexture>)>,
            ),
        >,
        Query<&mut UiColor, With<PauseButton>>,
    )>,
) {
    let interaction = queries
        .p0()
        .iter()
        .fold(Interaction::None, |acc, x| match acc {
            Interaction::Clicked => acc,
            Interaction::Hovered => match x {
                Interaction::Clicked => Interaction::Clicked,
                Interaction::Hovered => Interaction::Hovered,
                Interaction::None => Interaction::Hovered,
            },
            Interaction::None => *x,
        });
    let mut pause_query = queries.p1();

    let mut button_mat = if let Some(mat) = pause_query.iter_mut().next() {
        mat
    } else {
        return;
    };
    match interaction {
        Interaction::Clicked => {
            if gui_state.water_speed >= 1 {
                gui_state.water_speed = 0;
                *button_mat = UiColor(GUI_STYLE.button_pressed_color);
            } else {
                *button_mat = UiColor(GUI_STYLE.button_hover_color);

                gui_state.water_speed = 1;
            }
        }
        Interaction::Hovered => {
            *button_mat = UiColor(GUI_STYLE.button_hover_color);
        }
        Interaction::None => {
            if gui_state.water_speed >= 1 {
                *button_mat = UiColor(GUI_STYLE.button_normal_color);
            } else {
                *button_mat = UiColor(GUI_STYLE.button_pressed_color);
            }
        }
    };
}
fn leave_button(
    mut state: ResMut<State<GameState>>,
    mut queries: ParamSet<(
        Query<&Interaction, (With<Interaction>, Or<(With<LeaveButton>, With<LeaveText>)>)>,
        Query<&mut UiColor, With<LeaveButton>>,
    )>,
) {
    let interaction = queries
        .p0()
        .iter()
        .fold(Interaction::None, |acc, x| match acc {
            Interaction::Clicked => Interaction::Clicked,
            Interaction::Hovered => match x {
                Interaction::Clicked => Interaction::Clicked,
                _ => Interaction::Hovered,
            },
            Interaction::None => *x,
        })
        .clone();
    for mut material in queries.p1().iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = UiColor(GUI_STYLE.button_pressed_color);

                state.set(GameState::Menu).expect("failed to set state");
            }
            Interaction::Hovered => {
                *material = UiColor(GUI_STYLE.button_hover_color);
            }
            Interaction::None => {
                *material = UiColor(GUI_STYLE.button_normal_color);
            }
        };
    }
}

fn play_button(
    mut gui_state: ResMut<GuiState>,
    mut queries: ParamSet<(
        Query<
            &Interaction,
            (
                Changed<Interaction>,
                Or<(With<PlayButton>, With<PlayTexture>)>,
            ),
        >,
        Query<&mut UiColor, With<PlayButton>>,
    )>,
) {
    let interaction = queries.p0().iter().next().copied();

    let mut play_query = queries.p1();
    let mut play_material = if let Some(mat) = play_query.iter_mut().next() {
        mat
    } else {
        return;
    };
    if gui_state.water_speed == 0 {
        *play_material = UiColor(GUI_STYLE.button_normal_color);
    }

    if let Some(interaction) = interaction {
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
                gui_state.water_speed = new_speed;
                *play_material = UiColor(GUI_STYLE.button_pressed_color);
            }
            Interaction::Hovered => {
                *play_material = UiColor(GUI_STYLE.button_hover_color);
            }
            Interaction::None => {}
        }
    }
}
fn show_velocity_button(
    mut gui_state: ResMut<GuiState>,
    mut queries: ParamSet<(
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ShowVelocities>)>,
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ShowWater>)>,
    )>,
) {
    for (interation, mut material) in queries.p0().iter_mut() {
        match *interation {
            Interaction::Clicked => {
                gui_state.show_velocities = !gui_state.show_velocities;
                if gui_state.show_velocities {
                    *material = UiColor(GUI_STYLE.button_pressed_color);
                } else {
                    *material = UiColor(GUI_STYLE.button_normal_color);
                }
            }
            Interaction::Hovered => {
                if !gui_state.show_velocities {
                    *material = UiColor(GUI_STYLE.button_hover_color);
                }
            }
            Interaction::None => {
                if gui_state.show_velocities {
                    *material = UiColor(GUI_STYLE.button_pressed_color);
                }
            }
        }
    }
    for (interation, mut material) in queries.p1().iter_mut() {
        match *interation {
            Interaction::Clicked => {
                gui_state.show_water = !gui_state.show_water;
                if gui_state.show_water {
                    *material = UiColor(GUI_STYLE.button_pressed_color);
                } else {
                    *material = UiColor(GUI_STYLE.button_normal_color);
                }
            }
            Interaction::Hovered => {
                if !gui_state.show_water {
                    *material = UiColor(GUI_STYLE.button_hover_color);
                }
            }
            Interaction::None => {
                if gui_state.show_water {
                    *material = UiColor(GUI_STYLE.button_pressed_color);
                }
            }
        }
    }
}
fn save_water(
    solver_query: Query<&PreferredSolver, With<PreferredSolver>>,
    mut query: Query<(&Interaction, &mut UiColor), (With<SaveWaterButton>, Changed<Interaction>)>,
) {
    for (interaction, mut mat) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *mat = UiColor(GUI_STYLE.button_pressed_color);

                if let Some(solver) = solver_query.iter().next() {
                    crate::file_save::save(&solver.numpy_data());
                }
            }
            Interaction::Hovered => {
                *mat = UiColor(GUI_STYLE.button_hover_color);
            }
            Interaction::None => {
                *mat = UiColor(GUI_STYLE.button_normal_color);
            }
        }
    }
}
fn add_box_button(
    mut commands: Commands,

    aabb_material: Res<AABBMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    solver_query: Query<&PreferredSolver, ()>,
    mut queries: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<AddBoxButton>)>,
) {
    for (interation, mut material) in queries.iter_mut() {
        match *interation {
            Interaction::Clicked => {
                let water = if let Some(water) = solver_query.iter().next() {
                    water
                } else {
                    error!("failed to find water entity");
                    return;
                };
                let mean_h = water.mean_height();
                let mut aabb_transform = Transform::default();
                aabb_transform.translation.x = WATER_SIZE / 2.0;
                aabb_transform.translation.y = WATER_SIZE / 2.0;
                build_barrier(
                    &mut commands,
                    aabb_barrier_from_transform(aabb_transform, water),
                    &aabb_material,
                    &mut meshes,
                    mean_h,
                    Vector2::new(water.water_h().x(), water.water_h().y()),
                );
                *material = UiColor(GUI_STYLE.button_pressed_color);
            }
            Interaction::Hovered => {
                *material = UiColor(GUI_STYLE.button_hover_color);
            }
            Interaction::None => {
                *material = UiColor(GUI_STYLE.button_normal_color);
            }
        }
    }
}

fn solve_info(
    mut _commands: Commands,
    solve_query: Query<&SolveInfoVec, With<WaterMarker>>,
    mut query: Query<&mut Text, With<SolveInfoLabel>>,
) {
    if let Some(info_vec) = solve_query.iter().next() {
        let formatted_str = info_vec
            .data
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
    water_query: Query<&PreferredSolver, With<WaterMarker>>,
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
fn cleanup_ui(mut commands: Commands, mut query: Query<Entity, With<GameEntity>>) {
    for e in query.iter_mut() {
        commands.entity(e).despawn_recursive();
    }
}
