use crate::prelude::{
    FontAssets, GameEntity, ShowVelocities, ShowWater, SolveInfoLabel, GUI_STYLE,
};
use bevy::prelude::*;
/// constructs time bar at bottom of game
pub fn build_playbar(parent: &mut ChildBuilder<'_, '_, '_>, font_assets: &Res<FontAssets>) {
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
                            let fira_sans = &font_assets.fira_sans;
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
                                            font: fira_sans.clone_weak(),
                                            font_size: 30.0,
                                            color: GUI_STYLE.button_text_color.clone(),
                                        },
                                    ),
                                    ..Default::default()
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
                                .insert(ShowWater)
                                .insert(GameEntity);
                        });
                });
        });
}
