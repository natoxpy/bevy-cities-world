use bevy::prelude::*;
// use serde_json::Value;
use std::fs;

use super::data::{ContentPointType, PointType};

#[derive(Component)]
pub struct Bg3dCamera;

#[derive(Component)]
pub struct BgObj;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(50., 50., 50.).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
        Bg3dCamera,
    ));

    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 100000.,
            range: 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(15., 25., 15.),
        ..default()
    },));
}

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let content = fs::read_to_string("./apps/start_menu/data/menu_city.json")
        .expect("Data for the BG menu not found!");

    let city: Vec<crate::menu_background::data::Point> =
        serde_json::from_str(content.as_str()).unwrap();

    let grass: Handle<Scene> = asset_server.load("./models/grass_flat.glb#Scene0");
    let tree1: Handle<Scene> = asset_server.load("./models/nature/tree_01.glb#Scene0");

    for point in city {
        let tf = Transform::from_xyz(point.position.x * 21., 0., point.position.y * 21.);

        match point.has {
            PointType::Grass => {
                commands.spawn(SceneBundle {
                    scene: grass.clone(),
                    transform: tf,
                    ..default()
                });
            }
        }

        for content_point in point.content {
            let mut content_tf = tf.clone();

            content_tf.translation +=
                Quat::from_rotation_y((90. as f32).to_radians() as f32) * content_point.position.to_vec3();

            match content_point.has {
                ContentPointType::Tree1 => {
                    commands.spawn(SceneBundle {
                        scene: tree1.clone(),
                        transform: content_tf,
                        ..default()
                    });
                }
            }
        }
    }
}
