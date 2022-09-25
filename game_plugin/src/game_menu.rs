use bevy::prelude::*;

pub struct GuiStyle {
    pub button_normal_color: Color,
    pub button_hover_color: Color,
    pub button_pressed_color: Color,
    pub side_panel_color: Color,
    pub bottom_panel_color: Color,
    pub bottom_subpanel_color: Color,
    pub button_text_color: Color,
    pub text_color: Color,
    pub main_menu_bg_color: Color,
    pub nav_bar_bg_color: Color,
    pub nav_bar_button_normal: Color,
    pub nav_bar_button_clicked: Color,
    pub nav_bar_button_hover: Color,
    pub page_color: Color,
}
pub const GUI_STYLE: GuiStyle = GuiStyle {
    button_normal_color: Color::Rgba {
        red: 0.25,
        green: 0.25,
        blue: 0.25,
        alpha: 1.0,
    },
    button_hover_color: Color::Rgba {
        red: 0.4,
        green: 0.4,
        blue: 0.4,
        alpha: 1.0,
    },
    button_pressed_color: Color::Rgba {
        red: 0.3,
        green: 0.3,
        blue: 0.3,
        alpha: 1.0,
    },
    side_panel_color: Color::Rgba {
        red: 0.2,
        green: 0.2,
        blue: 0.2,
        alpha: 1.0,
    },
    bottom_panel_color: Color::Rgba {
        red: 0.17,
        green: 0.17,
        blue: 0.17,
        alpha: 1.0,
    },
    text_color: Color::Rgba {
        red: 0.9,
        green: 0.6,
        blue: 0.2,
        alpha: 1.0,
    },
    bottom_subpanel_color: Color::Rgba {
        red: 0.9,
        green: 0.2,
        blue: 0.2,
        alpha: 0.0,
    },
    button_text_color: Color::Rgba {
        red: 0.9,
        green: 0.6,
        blue: 0.2,
        alpha: 1.0,
    },
    main_menu_bg_color: Color::Rgba {
        red: 0.1,
        green: 0.1,
        blue: 0.1,
        alpha: 1.0,
    },
    nav_bar_bg_color: Color::Rgba {
        red: 0.3,
        green: 0.3,
        blue: 0.3,
        alpha: 1.0,
    },
    nav_bar_button_normal: Color::Rgba {
        red: 0.33,
        green: 0.33,
        blue: 0.33,
        alpha: 1.0,
    },
    nav_bar_button_hover: Color::Rgba {
        red: 0.35,
        green: 0.35,
        blue: 0.35,
        alpha: 1.0,
    },
    nav_bar_button_clicked: Color::Rgba {
        red: 0.31,
        green: 0.31,
        blue: 0.31,
        alpha: 1.0,
    },
    page_color: Color::Rgba {
        red: 0.9,
        green: 0.9,
        blue: 0.9,
        alpha: 1.0,
    },
};
#[derive(Component)]
pub struct GameMenu;

/// Marks viscocoty change text button
#[derive(Component)]
pub struct ViscocityChange;
#[derive(Component)]
pub struct SolveInfoLabel;

pub struct dep_ButtonMaterial {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
    pub main_menu_bg: Handle<ColorMaterial>,
    pub side_panel: Handle<ColorMaterial>,
    pub bottom_panel: Handle<ColorMaterial>,
    pub bottom_subpanel: Handle<ColorMaterial>,
    pub button_text: Handle<ColorMaterial>,
    pub nav_bar_bg: Handle<ColorMaterial>,
    pub nav_bar_button_normal: Handle<ColorMaterial>,
    pub nav_bar_button_hover: Handle<ColorMaterial>,
    pub nav_bar_button_clicked: Handle<ColorMaterial>,
    pub page: Handle<ColorMaterial>,
}
impl FromWorld for dep_ButtonMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            normal: materials.add(GUI_STYLE.button_normal_color.into()),
            hovered: materials.add(GUI_STYLE.button_hover_color.into()),
            pressed: materials.add(GUI_STYLE.button_pressed_color.into()),
            main_menu_bg: materials.add(GUI_STYLE.main_menu_bg_color.into()),
            side_panel: materials.add(GUI_STYLE.side_panel_color.into()),
            bottom_panel: materials.add(GUI_STYLE.bottom_panel_color.into()),
            bottom_subpanel: materials.add(GUI_STYLE.bottom_subpanel_color.into()),
            button_text: materials.add(GUI_STYLE.button_text_color.into()),
            nav_bar_bg: materials.add(GUI_STYLE.nav_bar_bg_color.into()),
            nav_bar_button_normal: materials.add(GUI_STYLE.nav_bar_button_normal.into()),
            nav_bar_button_hover: materials.add(GUI_STYLE.nav_bar_button_hover.into()),
            nav_bar_button_clicked: materials.add(GUI_STYLE.nav_bar_button_clicked.into()),
            page: materials.add(GUI_STYLE.page_color.into()),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedDirection {
    Increasing,
    Decreasing,
}
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub struct GuiState {
    pub show_velocities: bool,
    pub show_water: bool,
    pub water_speed: u32,
    /// whether or not to increase speed when play button is clicked
    pub speed_direction: SpeedDirection,
}
impl Default for GuiState {
    fn default() -> Self {
        Self {
            show_velocities: false,
            show_water: true,
            water_speed: 0,
            speed_direction: SpeedDirection::Increasing,
        }
    }
}
pub const MAX_WATER_SPEED: u32 = 64;
/// Marker for play button
#[derive(Component)]
pub struct PlayButton;
#[derive(Component)]
pub struct PauseButton;
#[derive(Component)]
pub struct PauseTexture;
#[derive(Component)]
pub struct PlayTexture;
#[derive(Component)]
pub struct ShowSpeed;
#[derive(Component)]
pub struct AddBoxButton;
/// Marks Show Velocities button
#[derive(Component)]
pub struct ShowVelocities;
/// Marks show water
#[derive(Component)]
pub struct ShowWater;
#[derive(Component)]
pub struct LeaveButton;
#[derive(Component)]
pub struct LeaveText;
#[derive(Component)]
pub struct SaveWaterButton;
/// marks that belongs to game::playing state. will be destroyed at end of this state
#[derive(Component)]
pub struct GameEntity;
