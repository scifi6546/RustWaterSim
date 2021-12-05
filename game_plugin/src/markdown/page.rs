use super::{build_gui, FontAssets, RootNode};
use crate::prelude::ButtonMaterial;
use pulldown_cmark::{Event, Parser, Tag};

use crate::GameState;
use bevy::prelude::*;
/// Overall document
pub struct Document {
    pub pages: Vec<Page>,
}
/// Single page in document
pub struct Page {
    pub title: String,
    pub content: Vec<PageElement>,
}
pub struct PageState {
    page_index: usize,
}
pub enum PageElement {
    Heading { level: u32, text: String },
    Image { path: String },
    ParagraphBreak,
    SoftBreak,
    Text(String),
}
enum ParseState {
    None,
    Heading { level: u32 },
}
pub fn load_markdown() -> Document {
    let markdown_data = include_str!("../../../assets/write_up/main.md");
    let pages = markdown_data.split("@P");
    let pages = pages
        .map(|text| {
            let mut state = ParseState::None;
            let parser = Parser::new(text);
            let mut content = vec![];
            let mut title = "".to_string();
            for event in parser {
                match event {
                    Event::Start(tag) => match tag {
                        Tag::Paragraph => {

                            //content.push(PageElement::ParagraphBreak);
                        }
                        Tag::Heading(level) => {
                            state = ParseState::Heading { level };
                        }
                        Tag::Image(_, path, _) => {
                            content.push(PageElement::Image {
                                path: path.to_string(),
                            });
                        }
                        _ => panic!("unsupported tag: {:?}", tag),
                    },
                    Event::End(_) => {
                        state = ParseState::None;
                    }
                    Event::SoftBreak => {
                        // content.push(PageElement::SoftBreak);
                    }

                    Event::Text(text) => match state {
                        ParseState::Heading { level } => {
                            if title == "" {
                                title = text.to_string();
                            }
                            content.push(PageElement::Heading {
                                level,
                                text: text.to_string(),
                            });
                        }
                        ParseState::None => {
                            content.push(PageElement::Text(text.to_string()));
                        }
                    },
                    _ => panic!("unsupported event: {:?}", event),
                }
            }
            Page { title, content }
        })
        .collect();
    Document { pages }
}
pub fn setup(mut commands: Commands) {
    commands.insert_resource(load_markdown());
    commands.insert_resource(PageState { page_index: 0 });
}
pub fn button(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    document: Res<Document>,
    asset_server: Res<AssetServer>,
    mut page_state: ResMut<PageState>,
    materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterial>,

    mut state: ResMut<State<GameState>>,
    mut query: Query<
        (&mut Handle<ColorMaterial>, &Interaction, &super::PageButton),
        (With<super::PageButtonMarker>, Changed<Interaction>),
    >,

    root_query: Query<Entity, With<RootNode>>,
) {
    for (mut mat, interaction, page) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                page_state.page_index = page.index;
                *mat = button_materials.nav_bar_button_clicked.clone();
                if *state.current() == GameState::Page {
                    for root in root_query.iter() {
                        commands.entity(root).despawn_recursive();
                    }
                    setup_page(
                        commands,
                        font_assets,
                        document,
                        asset_server,
                        page_state,
                        materials,
                        button_materials,
                    );
                    return;
                } else {
                    state.set(GameState::Page).expect("failed to set state");
                }
            }
            Interaction::Hovered => {
                *mat = button_materials.nav_bar_button_hover.clone();
            }
            Interaction::None => {
                *mat = button_materials.nav_bar_button_normal.clone();
            }
        }
    }
}
pub fn setup_page(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    document: Res<Document>,
    asset_server: Res<AssetServer>,
    page_state: ResMut<PageState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterial>,
) {
    build_gui(
        &mut commands,
        &mut materials,
        &font_assets,
        &button_materials,
        &document,
        |materials, _, buttom_materials, parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_content: AlignContent::FlexStart,
                        align_self: AlignSelf::FlexStart,
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        size: Size::new(Val::Percent(60.0), Val::Auto),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    material: buttom_materials.page.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    for element in document.pages[page_state.page_index].content.iter() {
                        match element {
                            PageElement::Heading { level, text } => {
                                parent.spawn_bundle(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection {
                                            value: text.clone(),
                                            style: TextStyle {
                                                font: font_assets.fira_sans.clone(),
                                                font_size: match level {
                                                    1 => 40.0,
                                                    2 => 20.0,
                                                    _ => 15.0,
                                                },
                                                color: Color::BLACK,
                                            },
                                        }],
                                        alignment: Default::default(),
                                    },
                                    style: Style {
                                        size: Size::new(Val::Px(500.0), Val::Auto),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            }
                            PageElement::Text(text) => {
                                parent.spawn_bundle(TextBundle {
                                    text: Text {
                                        sections: vec![TextSection {
                                            value: text.clone(),
                                            style: TextStyle {
                                                font: font_assets.fira_sans.clone(),
                                                font_size: 13.0,
                                                color: Color::BLACK,
                                            },
                                        }],
                                        alignment: Default::default(),
                                    },
                                    style: Style {
                                        size: Size::new(Val::Px(700.0), Val::Auto),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            }
                            PageElement::ParagraphBreak => {
                                error!("todo: paragraph break")
                            }
                            PageElement::Image { path } => {
                                info!("path: {}", path);
                                let size = match path.as_str() {
                                    "lab.png" => {
                                        info!("lab size");
                                        Size::new(Val::Px(1707.0 / 5.0), Val::Px(1280.0 / 5.0))
                                    }
                                    "scheme.png" => {
                                        Size::new(Val::Px(960.0 / 2.0), Val::Px(540.0 / 2.0))
                                    }

                                    "double_slit.png" => {
                                        Size::new(Val::Px(992.0 / 3.0), Val::Px(685.0 / 3.0))
                                    }
                                    "equation2.png" => {
                                        Size::new(Val::Px(200.0 / 1.0), Val::Px(146.0 / 1.0))
                                    }
                                    "equation3.png" => {
                                        Size::new(Val::Px(549.0 / 1.0), Val::Px(242.0 / 1.0))
                                    }
                                    "equation4.png" => {
                                        Size::new(Val::Px(96.0 / 1.0), Val::Px(41.0 / 1.0))
                                    }
                                    "equation5.png" => {
                                        Size::new(Val::Px(212.0 / 1.0), Val::Px(41.0 / 1.0))
                                    }
                                    "equation6.png" => {
                                        Size::new(Val::Px(243.0 / 1.0), Val::Px(41.0 / 1.0))
                                    }

                                    "equation7.png" => {
                                        Size::new(Val::Px(290.0 / 1.0), Val::Px(50.0 / 1.0))
                                    }
                                    _ => Size::new(Val::Px(1.0), Val::Px(1.0)),
                                };
                                let path = "write_up/".to_owned() + path;

                                parent.spawn_bundle(ImageBundle {
                                    style: Style {
                                        size,
                                        ..Default::default()
                                    },
                                    material: materials
                                        .add(asset_server.load(path.as_str()).into()),
                                    ..Default::default()
                                });
                            }
                            PageElement::SoftBreak => {
                                error!("todo soft break");
                            }
                        }
                    }
                });
        },
    );
}
