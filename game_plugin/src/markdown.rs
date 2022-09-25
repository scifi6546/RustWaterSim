use super::prelude::{dep_ButtonMaterial, GUI_STYLE};
use super::GameState;
use crate::loading::FontAssets;
use bevy::prelude::*;
pub struct DocumentPlugin;
mod page;
pub use page::{Document, Page};
impl Plugin for DocumentPlugin {
    fn build(&self, app: &mut App) {
        info!("building document plugin");
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(page::setup)
                .label(BuiltParentLabel),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(nav_system)
                .with_system(page::button),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Sandbox)
                .with_system(nav_system)
                .with_system(page::button),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Page)
                .with_system(nav_system)
                .with_system(page::button),
        );

        app.add_system_set(SystemSet::on_enter(GameState::Page).with_system(page::setup_page));

        app.add_system_set(SystemSet::on_exit(GameState::Page).with_system(despawn_gui));
    }
}
#[derive(Copy, Clone, Component, Debug)]
pub struct RootNode;
#[derive(Debug, Hash, PartialEq, Eq, SystemLabel, Copy, Clone, Component)]
pub struct BuiltParentLabel;
/// root note of gui
#[derive(Copy, Clone, Component, Debug)]
pub struct GuiParent;
/// parent of document gui
#[derive(Copy, Clone, Component, Debug)]
pub struct DocumentGuiParent;
#[derive(Copy, Clone, Component, Debug)]
pub struct SimulationButton;
#[derive(Copy, Clone, Component, Debug)]
pub struct PageButton {
    pub index: usize,
}
#[derive(Copy, Clone, Component, Debug)]
pub struct PageButtonMarker;
pub struct GuiEntities {
    pub nav_bar_entity: Entity,
}
pub fn build_gui(
    commands: &mut Commands,
    font_assets: &Res<FontAssets>,
    document: &Res<Document>,
    asset_server: &Res<AssetServer>,
    f: impl FnOnce(&Res<AssetServer>, &mut ChildBuilder<'_, '_, '_>),
) -> GuiEntities {
    let mut nav_bar_entity = None;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_content: AlignContent::FlexStart,
                align_self: AlignSelf::Center,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
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
        .insert(RootNode)
        .with_children(|parent| {
            let a = build_navbar(parent, &font_assets, document);
            nav_bar_entity = Some(a);
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_content: AlignContent::Stretch,
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Stretch,

                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::ColumnReverse,
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
                .insert(GuiParent)
                .with_children(|parent| {
                    f(asset_server, parent);
                });
        });
    GuiEntities {
        nav_bar_entity: nav_bar_entity.unwrap(),
    }
}

fn build_navbar<'a>(
    parent: &mut ChildBuilder<'_, '_, '_>,
    font_assets: &Res<FontAssets>,
    document: &Res<Document>,
) -> Entity {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                flex_direction: FlexDirection::Row,

                ..Default::default()
            },
            color: UiColor(GUI_STYLE.nav_bar_bg_color),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: UiColor(GUI_STYLE.nav_bar_button_normal),
                    ..Default::default()
                })
                .insert(SimulationButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Demo".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 30.0,
                                    color: GUI_STYLE.button_text_color,
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    });
                });
            for (index, page) in document.pages.iter().enumerate() {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: UiColor(GUI_STYLE.nav_bar_button_normal),
                        ..Default::default()
                    })
                    .insert(PageButton { index })
                    .insert(PageButtonMarker)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: page.title.clone(),
                                    style: TextStyle {
                                        font: font_assets.fira_sans.clone(),
                                        font_size: 30.0,
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
    parent.parent_entity()
}
pub fn nav_system(
    material: Res<dep_ButtonMaterial>,
    mut state: ResMut<State<GameState>>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<SimulationButton>),
    >,
) {
    for (interaction, mut mat) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *mat = material.nav_bar_button_clicked.clone();
                state.set(GameState::Menu).expect("failed to set state");
            }
            Interaction::Hovered => {
                *mat = material.nav_bar_button_hover.clone();
            }
            Interaction::None => {
                *mat = material.nav_bar_button_normal.clone();
            }
        }
    }
}
/// removes gui
pub fn despawn_gui(mut commands: Commands, root_query: Query<Entity, With<RootNode>>) {
    for root in root_query.iter() {
        commands.entity(root).despawn_recursive();
    }
}
