use crate::loading::FontAssets;
use crate::prelude::{ButtonMaterial, CONDITIONS, GUI_STYLE};
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterial>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(conditions_button.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(clear_ui.system()));
    }
}

#[derive(Debug, Clone)]
pub struct SelectStartupInfo {
    /// index in CONDITIONS to spawn
    pub index: usize,
    /// name of startup condition.
    pub name: String,
}
struct SelectStartup;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterial>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                //size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.main_menu_bg.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            for (index, startup) in (&CONDITIONS).iter().enumerate() {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(420.0), Val::Px(50.0)),
                            margin: Rect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: button_materials.normal.clone(),
                        ..Default::default()
                    })
                    .insert(SelectStartupInfo {
                        index,
                        name: startup.name.to_string(),
                    })
                    .insert(SelectStartup)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
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
                        });
                    });
            }
        });
}

fn conditions_button(
    mut commands: Commands,
    button_materials: Res<ButtonMaterial>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &SelectStartupInfo),
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
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

/// clears ui of all ui items
fn clear_ui(
    mut commands: Commands,
    text_query: Query<Entity, Or<(With<Text>, With<Node>, With<Button>)>>,
) {
    for entity in text_query.iter() {
        commands.entity(entity).despawn();
    }
}
