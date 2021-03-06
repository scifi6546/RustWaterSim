use super::prelude::{ButtonMaterial, GUI_STYLE};
use super::GameState;
use crate::loading::FontAssets;
use bevy::prelude::*;
pub struct DocumentPlugin;
mod page;
pub use page::{Document, Page};
impl Plugin for DocumentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        info!("building document plugin");
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(page::setup.system())
                .label(BuiltParentLabel),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(nav_system.system())
                .with_system(page::button.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(nav_system.system())
                .with_system(page::button.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Page)
                .with_system(nav_system.system())
                .with_system(page::button.system()),
        );

        app.add_system_set(
            SystemSet::on_enter(GameState::Page).with_system(page::setup_page.system()),
        );

        app.add_system_set(SystemSet::on_exit(GameState::Page).with_system(despawn_gui.system()));
    }
}
pub struct RootNode;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct BuiltParentLabel;
/// root note of gui
pub struct GuiParent;
/// parent of document gui
pub struct DocumentGuiParent;
pub struct SimulationButton;
pub struct PageButton {
    pub index: usize,
}
pub struct PageButtonMarker;
pub fn build_gui(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    font_assets: &Res<FontAssets>,
    button_material: &Res<ButtonMaterial>,
    document: &Res<Document>,

    f: impl FnOnce(
        &mut ResMut<Assets<ColorMaterial>>,
        &Res<FontAssets>,
        &Res<ButtonMaterial>,
        &mut ChildBuilder<'_, '_>,
    ),
) {
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
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(RootNode)
        .with_children(|parent| {
            build_navbar(parent, &font_assets, document, &button_material);
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
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .insert(GuiParent)
                .with_children(|parent| {
                    f(materials, font_assets, button_material, parent);
                });
        });
}

fn build_navbar<'a>(
    parent: &mut ChildBuilder<'_, 'a>,
    font_assets: &Res<FontAssets>,
    document: &Res<Document>,
    materials: &ButtonMaterial,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                flex_direction: FlexDirection::Row,

                ..Default::default()
            },
            material: materials.nav_bar_bg.clone(),
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
                    material: materials.nav_bar_button_normal.clone(),
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
                            margin: Rect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: materials.nav_bar_button_normal.clone(),
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
}
pub fn nav_system(
    material: Res<ButtonMaterial>,
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
