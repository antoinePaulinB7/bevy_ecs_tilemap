use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_size = TilemapSize { x: 32, y: 32 };
    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_entity = commands.spawn().id();

    for x in 0..32u32 {
        for y in 0..32u32 {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 0.0),
            ..Default::default()
        })
        .insert(LastUpdate::default());
}

#[derive(Default, Component)]
struct LastUpdate {
    value: f64,
}

fn remove_tiles(
    time: Res<Time>,
    mut last_update_query: Query<(&mut LastUpdate, &TileStorage)>,
    mut tile_query: Query<&mut TileVisible>,
) {
    let current_time = time.seconds_since_startup();
    for (mut last_update, tile_storage) in last_update_query.iter_mut() {
        // Remove a tile every half second.
        if (current_time - last_update.value) > 0.1 {
            let mut random = thread_rng();
            let position = TilePos {
                x: random.gen_range(0..32),
                y: random.gen_range(0..32),
            };

            // Instead of removing the tile entity we want to hide the tile by removing the Visible component.
            if let Some(tile_entity) = tile_storage.get(&position) {
                let mut visibility = tile_query.get_mut(tile_entity).unwrap();
                visibility.0 = !visibility.0;
            }

            last_update.value = current_time;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Visibility Example"),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .add_system(remove_tiles)
        .run();
}
