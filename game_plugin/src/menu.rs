use crate::loading::FontAssets;
use crate::prelude::{
    build_gui, despawn_gui, nav_system, BuiltParentLabel, ButtonMaterial, Document, GUI_STYLE,
};
use crate::GameState;
use bevy::prelude::*;
use water_sim::{InitialConditions, PreferredSolver};

pub struct MenuPlugin;
#[derive(Component)]
struct MenuItem;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonMaterial>()
            .add_startup_system(insert_conditions)
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_menu)
                    .after(BuiltParentLabel),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(conditions_button)
                    .with_system(nav_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(despawn_gui));
    }
}

#[derive(Debug, Clone, Component)]
pub struct SelectStartupInfo {
    /// index in CONDITIONS to spawn
    pub index: usize,
    /// name of startup condition.
    pub name: String,
}
#[derive(Debug, Clone, Component)]
struct SelectStartup;
fn insert_conditions(mut commands: Commands) {
    commands.insert_resource(water_sim::get_conditions::<PreferredSolver>())
}
fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    document: Res<Document>,
    conditions: Res<Vec<InitialConditions<water_sim::PreferredSolver>>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterial>,
) {
    build_gui(
        &mut commands,
        &mut materials,
        &font_assets,
        &button_materials,
        &document,
        |_, _, parent| {
            info!("building setup menu");
            info!("building based off of parent");
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        //size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(GUI_STYLE.main_menu_bg_color),
                    ..Default::default()
                })
                .insert(MenuItem)
                .with_children(|parent| {
                    for (index, startup) in (&conditions).iter().enumerate() {
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(420.0), Val::Px(50.0)),
                                    margin: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                color: UiColor(GUI_STYLE.button_normal_color),
                                ..Default::default()
                            })
                            .insert(SelectStartupInfo {
                                index,
                                name: startup.name.to_string(),
                            })
                            .insert(SelectStartup)
                            .insert(MenuItem)
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: startup.name.to_string(),
                                                style: TextStyle {
                                                    font: font_assets.fira_sans.clone(),
                                                    font_size: 40.0,
                                                    color: GUI_STYLE.button_text_color,
                                                },
                                            }],
                                            alignment: Default::default(),
                                        },
                                        ..Default::default()
                                    })
                                    .insert(MenuItem);
                            });
                    }
                });
        },
    );
}

fn conditions_button(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &SelectStartupInfo),
        (Changed<Interaction>, With<Button>, With<SelectStartup>),
    >,
) {
    for (interaction, mut material, select_info) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
                commands.insert_resource(select_info.clone());
            }
            Interaction::Hovered => {
                info!("hovered");
                *material = UiColor(GUI_STYLE.button_hover_color);
            }
            Interaction::None => {
                *material = UiColor(GUI_STYLE.button_normal_color);
            }
        }
    }
}
