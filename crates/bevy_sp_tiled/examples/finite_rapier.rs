//! This example shows a finite map with an external tileset and Rapier physics.

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_sp_tiled::prelude::*;

mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, helper::movement)
        .add_systems(Update, rotate_map)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let map_handle: Handle<TiledMap> = asset_server.load("finite.tmx");
    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        tiled_settings: TiledMapSettings {
            // This is the default, but we're setting it explicitly here for clarity.
            collision_object_names: ObjectNames::All,
            ..default()
        },
        ..Default::default()
    });
}

pub fn rotate_map(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tilemap: Query<(Entity, &mut Transform), With<TiledMapMarker>>,
) {
    for (_, mut transform) in tilemap.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyO) {
            transform.rotate_z(f32::to_radians(90.0 * time.delta_seconds()));
        }

        if keyboard_input.pressed(KeyCode::KeyP) {
            transform.rotate_z(f32::to_radians(-90.0 * time.delta_seconds()));
        }
    }
}
