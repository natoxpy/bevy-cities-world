use bevy::prelude::*;

pub mod actions;
pub mod menu_background;
pub mod setup;
pub mod states;

#[derive(Default)]
pub struct Menu;

pub fn init(app: &mut App) {
    app.add_plugins((Menu, menu_background::MenuBackgroundPlugin));
    app.add_state::<states::StartMenu>();
}

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (crate::setup::setup_core_menu, crate::setup::setup_camera),
        );

        app.add_systems(Update, test_buttons);
    }
}

use crate::setup::MenuButton;

fn test_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiImage,
            &mut BackgroundColor,
            &mut Style,
            &crate::setup::MenuButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut ui_image, mut bg, mut style, menu_btn) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                style.padding = UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(0.));

                *ui_image = UiImage::new(asset_server.load("./images/UI_btn.png"));
                *bg = Color::rgba(0.90, 0.90, 0.90, 1.0).into();
            }
            Interaction::Pressed => {
                match menu_btn {
                    MenuButton::Play => crate::actions::play(),
                    MenuButton::Settings => crate::actions::settings(),
                    MenuButton::Leave => crate::actions::leave(),
                }

                style.padding = UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(15.), Val::Px(0.));

                *ui_image = UiImage::new(asset_server.load("./images/UI_btn_pressed.png"));
            }
            Interaction::None => {
                style.padding = UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(0.));

                *ui_image = UiImage::new(asset_server.load("./images/UI_btn.png"));
                *bg = Color::rgba(1.0, 1.0, 1.0, 1.0).into();
            }
        }
    }
}
