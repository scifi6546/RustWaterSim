use crate::prelude::{
    build_gui as prelude_build_gui, build_playbar, ButtonMaterial, Document, FontAssets,
};
use bevy::prelude::*;

pub fn build_gui(
    mut commands: Commands,

    document: Res<Document>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_material: Res<ButtonMaterial>,
    font_assets: Res<FontAssets>,
) {
    prelude_build_gui(
        &mut commands,
        &mut materials,
        &font_assets,
        &button_material,
        &document,
        |font, parent| build_playbar(parent, font),
    )
}
