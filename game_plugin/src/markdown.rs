use super::prelude::{ButtonMaterial, GUI_STYLE};
use super::GameState;
use crate::loading::FontAssets;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
pub struct DocumentPlugin;
impl Plugin for DocumentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        info!("building document plugin");
        app.add_system_set(
            SystemSet::on_enter(GameState::BuildGui)
                .with_system(document.system())
                .with_system(build_parent_gui.system())
                .label(BuiltParentLabel),
        );
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct BuiltParentLabel;
/// root note of gui
pub struct GuiParent;
/// parent of document gui
pub struct DocumentGuiParent;
/// Overall document
pub struct Document {
    pub pages: Vec<Page>,
}
/// Single page in document
pub struct Page {
    pub title: String,
    pub text: String,
}
pub fn load_markdown() -> Document {
    let markdown_data = include_str!("../../assets/write_up/main.md");
    Document {
        pages: vec![Page {
            title: "Hello world".to_string(),
            text: "Here is some amazing text".to_string(),
        }],
    }
}
fn document(mut commands: Commands) {
    commands.insert_resource(load_markdown());
}
fn build_parent_gui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    font_assets: Res<FontAssets>,
    button_material: Res<ButtonMaterial>,
    mut state: ResMut<State<GameState>>,
) {
    info!("building parent gui");
    commands.spawn_bundle(UiCameraBundle::default());
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
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            build_navbar(parent, &font_assets, &button_material);
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
                    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 1.0).into()),
                    ..Default::default()
                })
                .insert(GuiParent);
        });
    state.set(GameState::Menu).unwrap();
}
fn build_navbar<'a>(
    parent: &mut ChildBuilder<'_, 'a>,
    font_assets: &Res<FontAssets>,
    materials: &ButtonMaterial,
) {
    info!("buiding nav bar");
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                ..Default::default()
            },
            material: materials.nav_bar_bg.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(420.0), Val::Px(50.0)),
                        margin: Rect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.nav_bar_button_normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "foo".to_string(),
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
        });
}
