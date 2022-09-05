use crate::prelude::{
    build_gui as prelude_build_gui, build_play_menu, ButtonMaterial, Document, FontAssets, GuiState,
};
use bevy::prelude::*;

pub fn build_gui(
    mut commands: Commands,
    gui_state: Res<GuiState>,
    document: Res<Document>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_material: Res<ButtonMaterial>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    prelude_build_gui(
        &mut commands,
        &mut materials,
        &font_assets,
        &button_material,
        &document,
        &asset_server,
        |font, asset, parent| build_play_menu(parent, asset, &gui_state),
    )
}
