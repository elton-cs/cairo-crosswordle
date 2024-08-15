use super::manual_bindgen::{Letter, LetterStatus};
use bevy::{prelude::*, utils::HashMap};

const MULTIPLIER: f32 = 20.;
const SCALE: Vec3 = Vec3::splat(1.3);

pub struct VisualizeImagePlugin;
impl Plugin for VisualizeImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_letter_to_image_map);
        app.add_systems(Update, (add_image_visualizer).chain());
    }
}

#[derive(Debug, Component)]
struct ImageVisual;

fn add_image_visualizer(
    mut commands: Commands,
    query: Query<(Entity, &Letter, &LetterStatus), Without<ImageVisual>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    letter_map: Res<LetterMap>,
) {
    let texture: Handle<Image> = asset_server.load("keys.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(15, 16), 5, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for (entity_id, letter, _letter_status) in query.iter() {
        let letter_value = letter.mock_hash.clone().to_string();
        let letter_index = letter_map.map.get(letter_value.as_str()).unwrap().clone() as usize;

        let sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                ((letter.position) as f32) * MULTIPLIER,
                ((0) as f32) * MULTIPLIER,
                1.,
            ))
            .with_scale(SCALE),
            texture: texture.clone(),
            ..default()
        };
        let texture_atlas = TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: letter_index,
        };

        commands
            .entity(entity_id)
            .insert((sprite, texture_atlas, ImageVisual));
    }
}

#[derive(Debug, Resource)]
struct LetterMap {
    map: HashMap<&'static str, u8>,
}

fn build_letter_to_image_map(mut commands: Commands) {
    let hashmap: HashMap<&str, u8> = [
        ("a", 0),
        ("b", 1),
        ("c", 2),
        ("d", 3),
        ("e", 4),
        ("f", 5),
        ("g", 6),
        ("h", 7),
        ("i", 8),
        ("j", 9),
        ("k", 10),
        ("l", 11),
        ("m", 12),
        ("n", 13),
        ("o", 14),
        ("p", 15),
        ("q", 16),
        ("r", 17),
        ("s", 18),
        ("t", 19),
        ("u", 20),
        ("v", 21),
        ("w", 22),
        ("x", 23),
        ("y", 24),
        ("z", 25),
        ("up", 26),
        ("right", 27),
        ("down", 28),
        ("left", 29),
        ("1", 30),
        ("2", 31),
        ("3", 32),
        ("4", 33),
        ("5", 34),
        ("6", 35),
        ("7", 36),
        ("8", 37),
        ("9", 38),
        ("0", 39),
    ]
    .into();

    commands.insert_resource(LetterMap { map: hashmap });
}
