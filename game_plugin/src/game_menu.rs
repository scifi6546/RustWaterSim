use crate::prelude::{
    aabb_barrier_from_transform, build_barrier, build_gui, despawn_gui, AABBMaterial, Document,
    FiniteSolver, SolveInfo, WaterMarker, WATER_SIZE,
};
use crate::{loading::FontAssets, GameState};
use bevy::prelude::*;
use nalgebra::Vector2;
use std::cmp::max;
pub struct GuiStyle {
    pub button_normal_color: Color,
    pub button_hover_color: Color,
    pub button_pressed_color: Color,
    pub side_panel_color: Color,
    pub bottom_panel_color: Color,
    pub bottom_subpanel_color: Color,
    pub button_text_color: Color,
    pub text_color: Color,
    pub main_menu_bg_color: Color,
    pub nav_bar_bg_color: Color,
    pub nav_bar_button_normal: Color,
    pub nav_bar_button_clicked: Color,
    pub nav_bar_button_hover: Color,
    pub page_color: Color,
}
pub const GUI_STYLE: GuiStyle = GuiStyle {
    button_normal_color: Color::Rgba {
        red: 0.25,
        green: 0.25,
        blue: 0.25,
        alpha: 1.0,
    },
    button_hover_color: Color::Rgba {
        red: 0.4,
        green: 0.4,
        blue: 0.4,
        alpha: 1.0,
    },
    button_pressed_color: Color::Rgba {
        red: 0.3,
        green: 0.3,
        blue: 0.3,
        alpha: 1.0,
    },
    side_panel_color: Color::Rgba {
        red: 0.2,
        green: 0.2,
        blue: 0.2,
        alpha: 1.0,
    },
    bottom_panel_color: Color::Rgba {
        red: 0.17,
        green: 0.17,
        blue: 0.17,
        alpha: 1.0,
    },
    text_color: Color::Rgba {
        red: 0.9,
        green: 0.6,
        blue: 0.2,
        alpha: 1.0,
    },
    bottom_subpanel_color: Color::Rgba {
        red: 0.9,
        green: 0.2,
        blue: 0.2,
        alpha: 0.0,
    },
    button_text_color: Color::Rgba {
        red: 0.9,
        green: 0.6,
        blue: 0.2,
        alpha: 1.0,
    },
    main_menu_bg_color: Color::Rgba {
        red: 0.1,
        green: 0.1,
        blue: 0.1,
        alpha: 1.0,
    },
    nav_bar_bg_color: Color::Rgba {
        red: 0.3,
        green: 0.3,
        blue: 0.3,
        alpha: 1.0,
    },
    nav_bar_button_normal: Color::Rgba {
        red: 0.33,
        green: 0.33,
        blue: 0.33,
        alpha: 1.0,
    },
    nav_bar_button_hover: Color::Rgba {
        red: 0.35,
        green: 0.35,
        blue: 0.35,
        alpha: 1.0,
    },
    nav_bar_button_clicked: Color::Rgba {
        red: 0.31,
        green: 0.31,
        blue: 0.31,
        alpha: 1.0,
    },
    page_color: Color::Rgba {
        red: 0.9,
        green: 0.9,
        blue: 0.9,
        alpha: 1.0,
    },
};

struct GameMenu;
pub struct GameMenuPlugin;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum GuiLabel {
    GuiCreate,
}
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterial>();
        app.init_resource::<GuiState>();
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(ui.system())
                .label(GuiLabel::GuiCreate),
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
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(show_speed.system())
                .with_system(solve_info.system())
                .with_system(leave_button.system())
                .with_system(save_water.system())
                .with_system(add_box_button.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(despawn_gui.system())
                .with_system(cleanup_ui.system()),
        );
    }
}
/// Marks viscocoty change text button
struct ViscocityChange;
struct SolveInfoLabel;

pub struct ButtonMaterial {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
    pub main_menu_bg: Handle<ColorMaterial>,
    pub side_panel: Handle<ColorMaterial>,
    pub bottom_panel: Handle<ColorMaterial>,
    pub bottom_subpanel: Handle<ColorMaterial>,
    pub button_text: Handle<ColorMaterial>,
    pub nav_bar_bg: Handle<ColorMaterial>,
    pub nav_bar_button_normal: Handle<ColorMaterial>,
    pub nav_bar_button_hover: Handle<ColorMaterial>,
    pub nav_bar_button_clicked: Handle<ColorMaterial>,
    pub page: Handle<ColorMaterial>,
}
impl FromWorld for ButtonMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            normal: materials.add(GUI_STYLE.button_normal_color.into()),
            hovered: materials.add(GUI_STYLE.button_hover_color.into()),
            pressed: materials.add(GUI_STYLE.button_pressed_color.into()),
            main_menu_bg: materials.add(GUI_STYLE.main_menu_bg_color.into()),
            side_panel: materials.add(GUI_STYLE.side_panel_color.into()),
            bottom_panel: materials.add(GUI_STYLE.bottom_panel_color.into()),
            bottom_subpanel: materials.add(GUI_STYLE.bottom_subpanel_color.into()),
            button_text: materials.add(GUI_STYLE.button_text_color.into()),
            nav_bar_bg: materials.add(GUI_STYLE.nav_bar_bg_color.into()),
            nav_bar_button_normal: materials.add(GUI_STYLE.nav_bar_button_normal.into()),
            nav_bar_button_hover: materials.add(GUI_STYLE.nav_bar_button_hover.into()),
            nav_bar_button_clicked: materials.add(GUI_STYLE.nav_bar_button_clicked.into()),
            page: materials.add(GUI_STYLE.page_color.into()),
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
const MAX_WATER_SPEED: u32 = 64;
/// Marker for play button
struct PlayButton;
struct PauseButton;
struct PauseTexture;
struct PlayTexture;
struct ShowSpeed;
struct AddBoxButton;
/// Marks Show Velocities button
struct ShowVelocities;
/// Marks show water
struct ShowWater;
struct LeaveButton;
struct LeaveText;
struct SaveWaterButton;
/// marks that belongs to game::playing state. will be destroyed at end of this state
pub struct GameEntity;
fn ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    document: Res<Document>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_material: Res<ButtonMaterial>,
    font_assets: Res<FontAssets>,
) {
    let gui_state = GuiState::default();
    build_gui(
        &mut commands,
        &mut materials,
        &font_assets,
        &button_material,
        &document,
        |materials, _, button_material, parent| {
            // root node
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
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .insert(GameEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                border: Rect::all(Val::Px(3.0)),
                                flex_grow: 2.0,
                                justify_content: JustifyContent::FlexStart,
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::FlexEnd,
                                align_self: AlignSelf::Stretch,
                                size: Size::new(Val::Auto, Val::Percent(100.0)),
                                ..Default::default()
                            },
                            material: button_material.side_panel.clone(),
                            ..Default::default()
                        })
                        .insert(GameEntity)
                        .with_children(|parent| {
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
                                            color: GUI_STYLE.text_color,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(SolveInfoLabel)
                                .insert(GameEntity);
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
                                .insert(GameEntity)
                                .insert(ShowVelocities)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            style: Style {
                                                align_self: AlignSelf::Center,
                                                margin: Rect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            text: Text::with_section(
                                                "Show Velocities",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 30.0,
                                                    color: GUI_STYLE.button_text_color.clone(),
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(GameEntity);
                                })
                                .insert(GameEntity);
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
                                .insert(GameEntity)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            style: Style {
                                                align_self: AlignSelf::Center,
                                                margin: Rect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            text: Text::with_section(
                                                "Show Water",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 30.0,
                                                    color: GUI_STYLE.button_text_color,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(GameEntity);
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
                                .insert(GameEntity)
                                .insert(AddBoxButton)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            style: Style {
                                                align_self: AlignSelf::Center,
                                                margin: Rect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            text: Text::with_section(
                                                "Add Box",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 30.0,
                                                    color: GUI_STYLE.button_text_color,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(GameEntity);
                                });
                            #[cfg(feature = "native")]
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
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
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
                            material: button_material.bottom_panel.clone(),
                            ..Default::default()
                        })
                        .insert(GameEntity)
                        .with_children(|parent| {
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
                                    material: button_material.bottom_subpanel.clone(),
                                    ..Default::default()
                                })
                                .insert(GameEntity)
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
                                        .insert(GameEntity)
                                        .with_children(|parent| {
                                            parent
                                                .spawn_bundle(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Px(50.0), Val::Auto),
                                                        ..Default::default()
                                                    },
                                                    material: materials.add(
                                                        asset_server
                                                            .load("textures/pause.png")
                                                            .into(),
                                                    ),
                                                    ..Default::default()
                                                })
                                                .insert(PauseTexture)
                                                .insert(GameEntity)
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
                                        .insert(GameEntity)
                                        .with_children(|parent| {
                                            parent
                                                .spawn_bundle(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Px(50.0), Val::Auto),
                                                        ..Default::default()
                                                    },
                                                    material: materials.add(
                                                        asset_server
                                                            .load("textures/play.png")
                                                            .into(),
                                                    ),
                                                    ..Default::default()
                                                })
                                                .insert(PlayTexture)
                                                .insert(GameEntity)
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
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 30.0,
                                                    color: GUI_STYLE.text_color,
                                                },
                                                Default::default(),
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
                                        margin: Rect::all(Val::Px(5.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    material: button_material.normal.clone(),

                                    ..Default::default()
                                })
                                .insert(LeaveButton)
                                .insert(GameEntity)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            style: Style {
                                                margin: Rect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            text: Text::with_section(
                                                "Exit".to_string(),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 30.0,
                                                    color: GUI_STYLE.button_text_color,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(LeaveText)
                                        .insert(GameEntity);
                                });
                        })
                        .insert(GameEntity);
                })
                .insert(GameEntity);
        },
    );
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
            &Interaction,
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
            Interaction::None => *x,
        });
    let mut button_mat = if let Some(mat) = queries.q1_mut().iter_mut().next() {
        mat
    } else {
        return;
    };
    match interaction {
        Interaction::Clicked => {
            if gui_state.water_speed >= 1 {
                gui_state.water_speed = 0;
                *button_mat = button_materials.pressed.clone();
            } else {
                *button_mat = button_materials.hovered.clone();
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
fn leave_button(
    button_materials: Res<ButtonMaterial>,
    mut state: ResMut<State<GameState>>,
    mut queries: QuerySet<(
        Query<&Interaction, (With<Interaction>, Or<(With<LeaveButton>, With<LeaveText>)>)>,
        Query<&mut Handle<ColorMaterial>, With<LeaveButton>>,
    )>,
) {
    let interaction = queries
        .q0()
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
    for mut material in queries.q1_mut().iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(GameState::Menu).expect("failed to set state");
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        };
    }
}

fn play_button(
    button_materials: Res<ButtonMaterial>,
    mut gui_state_query: Query<&mut GuiState, ()>,
    mut queries: QuerySet<(
        Query<
            &Interaction,
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
            (&Interaction, &mut Handle<ColorMaterial>),
            (Changed<Interaction>, With<ShowVelocities>),
        >,
        Query<(&Interaction, &mut Handle<ColorMaterial>), (Changed<Interaction>, With<ShowWater>)>,
    )>,
) {
    let gui_state = gui_state_query.iter_mut().next();
    if gui_state.is_none() {
        error!("gui state not found");
        return;
    }
    let mut gui_state = gui_state.unwrap();
    for (interation, mut material) in queries.q0_mut().iter_mut() {
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
    for (interation, mut material) in queries.q1_mut().iter_mut() {
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
fn save_water(
    button_materials: Res<ButtonMaterial>,
    solver_query: Query<&FiniteSolver, With<FiniteSolver>>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (With<SaveWaterButton>, Changed<Interaction>),
    >,
) {
    for (interaction, mut mat) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *mat = button_materials.pressed.clone();
                if let Some(solver) = solver_query.iter().next() {
                    crate::file_save::save(&solver.numpy_data());
                }
            }
            Interaction::Hovered => {
                *mat = button_materials.hovered.clone();
            }
            Interaction::None => {
                *mat = button_materials.normal.clone();
            }
        }
    }
}
fn add_box_button(
    mut commands: Commands,
    button_materials: Res<ButtonMaterial>,
    aabb_material: Res<AABBMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    solver_query: Query<&FiniteSolver, ()>,
    mut queries: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<AddBoxButton>),
    >,
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
                    Vector2::new(water.h().x(), water.h().y()),
                );
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
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
fn cleanup_ui(
    mut commands: Commands,
    mut query: Query<Entity, With<GameEntity>>,
    mut gui_state_query: Query<Entity, With<GuiState>>,
) {
    for e in query.iter_mut() {
        commands.entity(e).despawn_recursive();
    }
    for e in gui_state_query.iter_mut() {
        commands.entity(e).despawn();
    }
}
