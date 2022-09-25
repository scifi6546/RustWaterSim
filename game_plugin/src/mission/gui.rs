use super::DebugWin;
use crate::prelude::{
    build_gui as prelude_build_gui, build_play_menu, Document, FontAssets, GuiState, GUI_STYLE,
};
use bevy::prelude::*;

pub fn build_gui(
    mut commands: Commands,
    gui_state: Res<GuiState>,
    document: Res<Document>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    let entities = prelude_build_gui(
        &mut commands,
        &font_assets,
        &document,
        &asset_server,
        |asset, parent| {
            build_play_menu(parent, asset, &gui_state, |a, b| {
                b.spawn_bundle(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "t???",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: GUI_STYLE.text_color,
                        },
                    ),
                    ..Default::default()
                })
                .insert(DebugWin);
            })
        },
    );
}
