use bevy::prelude::*;

pub mod setup;
pub mod data;

pub struct MenuBackgroundPlugin;

impl Plugin for MenuBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup::setup, setup::setup_background));
    }
}
