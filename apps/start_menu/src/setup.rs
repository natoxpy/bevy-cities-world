use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers};

pub const UI_LAYER: RenderLayers = RenderLayers::layer(6);

#[derive(Component)]
pub enum MenuButton {
    Play,
    Settings,
    Leave,
}

impl MenuButton {
    pub fn is_play(&self) -> bool {
        match self {
            Self::Play => true,
            _ => false,
        }
    }

    pub fn is_settings(&self) -> bool {
        match self {
            Self::Settings => true,
            _ => false,
        }
    }

    pub fn is_leave(&self) -> bool {
        match self {
            Self::Leave => true,
            _ => false,
        }
    }
}

pub fn setup_core_menu(mut commands: Commands, asset_loader: Res<AssetServer>) {
    let blank_menu: Handle<Image> = asset_loader.load("./images/blank_menu.png");
    let btn_img: Handle<Image> = asset_loader.load("./images/UI_btn.png");

    let mouse_font: Handle<Font> = asset_loader.load("./fonts/mouse.otf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            UI_LAYER,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ImageBundle {
                        image: UiImage::new(blank_menu),
                        style: Style {
                            width: Val::Px(600.),
                            height: Val::Px(700.),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        ..default()
                    },
                    UI_LAYER,
                ))
                .with_children(|parent| {
                    // Title Wrapper
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(80.),
                                    height: Val::Px(100.),
                                    margin: UiRect::new(
                                        Val::Auto,
                                        Val::Auto,
                                        Val::Px(65.),
                                        Val::Px(0.),
                                    ),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            UI_LAYER,
                        ))
                        .with_children(|parent| {
                            // Title Text
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "THE GAME",
                                        TextStyle {
                                            font: mouse_font.clone(),
                                            font_size: 64.,
                                            color: Color::hex("B18759").unwrap(),
                                            ..default()
                                        },
                                    )
                                    .with_alignment(TextAlignment::Center),
                                    ..default()
                                },
                                UI_LAYER,
                            ));
                        });

                    add_button(
                        "PLAY",
                        parent,
                        btn_img.clone(),
                        mouse_font.clone(),
                        MenuButton::Play,
                    );
                    add_button(
                        "SETTINGS",
                        parent,
                        btn_img.clone(),
                        mouse_font.clone(),
                        MenuButton::Settings,
                    );
                    add_button(
                        "LEAVE",
                        parent,
                        btn_img.clone(),
                        mouse_font.clone(),
                        MenuButton::Leave,
                    );
                });
        });
}

fn add_button(
    content: &str,
    builder: &mut ChildBuilder<'_, '_, '_>,
    btn_img: Handle<Image>,
    font: Handle<Font>,
    action_component: impl Bundle,
) {
    builder
        .spawn((
            ButtonBundle {
                image: UiImage::new(btn_img),
                style: Style {
                    width: Val::Px(400.),
                    height: Val::Px(100.),
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(25.), Val::Px(20.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(1.0, 1.0, 1.0, 1.0).into(),
                ..default()
            },
            UI_LAYER,
            action_component,
        ))
        .with_children(|btn_wrapper| {
            btn_wrapper.spawn((
                TextBundle {
                    text: Text::from_section(
                        content,
                        TextStyle {
                            font: font.clone(),
                            font_size: 42.,
                            color: Color::hex("B18759").unwrap(),
                            ..default()
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
                UI_LAYER,
                Button,
            ));
        });
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        UI_LAYER,
    ));
}
