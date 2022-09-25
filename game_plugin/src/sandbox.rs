use crate::prelude::{
    build_gui as prelude_build_gui, build_play_menu, dep_ButtonMaterial, Document, FontAssets,
    GameState, GuiRunner, GuiState,
};

use bevy::prelude::*;

pub(crate) struct SandboxPlugin;
impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<dep_ButtonMaterial>()
            .init_resource::<GuiState>()
            .add_system_set(SystemSet::on_enter(GameState::Sandbox).with_system(build_gui))
            .add_plugin(GuiRunner {
                active_state: GameState::Sandbox,
            });
    }
}
fn build_gui(
    mut commands: Commands,
    gui_state: Res<GuiState>,
    document: Res<Document>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    prelude_build_gui(
        &mut commands,
        &font_assets,
        &document,
        &asset_server,
        |asset, parent| build_play_menu(parent, asset, &gui_state, |_, _| {}),
    );
}
