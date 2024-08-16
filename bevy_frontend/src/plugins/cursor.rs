use super::{
    constants::{CURSOR_IMAGE_INDEX, CURSOR_Z_HEIGHT, MULTIPLIER, SCALE},
    guess_letter::LetterGuess,
    manual_bindgen::Letter,
};
use bevy::prelude::*;

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_cursor, render_cursor).chain());
        app.add_systems(
            Update,
            (track_player_last_letter_input, update_cursor_render).chain(),
        );
    }
}

#[derive(Debug, Resource)]
struct PlayerCursor {
    position: u8,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Debug, Component)]
struct CursorTag;

fn spawn_cursor(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let cursor_texture: Handle<Image> = asset_server.load("new_keys.png");
    let cursor_layout = TextureAtlasLayout::from_grid(UVec2::new(15, 16), 6, 2, None, None);

    commands.insert_resource(PlayerCursor {
        position: 0,
        texture: cursor_texture,
        layout: texture_atlas_layouts.add(cursor_layout),
    });
}

fn render_cursor(query: Query<&Letter>, cursor: Res<PlayerCursor>, mut commands: Commands) {
    if cursor.is_added() {
        let cursor_sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                ((cursor.position) as f32) * MULTIPLIER,
                ((0) as f32) * MULTIPLIER,
                CURSOR_Z_HEIGHT,
            ))
            .with_scale(SCALE),
            texture: cursor.texture.clone(),
            ..default()
        };

        let cursor_texture_atlas = TextureAtlas {
            layout: cursor.layout.clone(),
            index: CURSOR_IMAGE_INDEX,
        };

        commands.spawn((cursor_sprite, cursor_texture_atlas, CursorTag));
    }
}

fn track_player_last_letter_input(
    mut cursor: ResMut<PlayerCursor>,
    player_guess_query: Query<&LetterGuess>,
) {
    let length = player_guess_query.get_single().unwrap().letter.len() as u8;
    if length > 0 {
        cursor.position = length - 1;
    } else {
        cursor.position = 0;
    }
}

fn update_cursor_render(
    mut query: Query<&mut Transform, With<CursorTag>>,
    cursor: Res<PlayerCursor>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x = ((cursor.position) as f32) * MULTIPLIER;
    }
}
