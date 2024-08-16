use super::image_visualize::LetterMap;
use crate::plugins::constants::{MULTIPLIER, SCALE};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

pub struct GuessLetterPlugin;
impl Plugin for GuessLetterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_empty_letter_guess);
        app.add_systems(Update, input_letter_guess);
        app.add_systems(PostUpdate, display_guess);
    }
}

#[derive(Debug, Component)]
pub struct LetterGuess {
    pub letters: Vec<String>,
    pub spawned_images: Vec<Entity>,
}

#[derive(Debug, Component)]
pub struct GuessLayer;

#[derive(Debug, Resource)]
pub struct GuessTextureAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

fn init_empty_letter_guess(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(LetterGuess {
        letters: vec![],
        spawned_images: vec![],
    });

    let guess_texture: Handle<Image> = asset_server.load("letters_only.png");
    let guess_layout = TextureAtlasLayout::from_grid(UVec2::new(15, 16), 5, 8, None, None);
    commands.insert_resource(GuessTextureAtlas {
        texture: guess_texture,
        layout: texture_atlas_layouts.add(guess_layout),
    });
}

fn input_letter_guess(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut letter_guess_query: Query<&mut LetterGuess>,
) {
    let mut letter = letter_guess_query.get_single_mut().unwrap();

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Character(c) => {
                let A: u32 = 'A'.into();
                let Z: u32 = 'Z'.into();
                let a: u32 = 'a'.into();
                let z: u32 = 'z'.into();
                let char_code: u32 = c.chars().next().unwrap().into();

                if (char_code >= a && char_code <= z) || (char_code >= A && char_code <= Z) {
                    println!("Typed: {}", c);
                    if letter.letters.len() < 5 {
                        letter.letters.push(c.to_lowercase().to_string());
                    } else {
                        println!("Already typed 5 letters!");
                    }
                } else {
                    println!("Invalid character: {}", c);
                }
            }
            Key::Backspace => {
                if !letter.letters.is_empty() {
                    letter.letters.pop();
                }
            }
            _ => {}
        }
    }
}

fn display_guess(
    mut letter_guess_query: Query<&mut LetterGuess>,
    resource: Res<GuessTextureAtlas>,
    letter_map: Res<LetterMap>,
    mut guess_query: Query<(Entity, &mut GuessLayer)>,
    mut commands: Commands,
) {
    let mut letter = letter_guess_query.get_single_mut().unwrap();
    // info!("Guess: {:?}", letter.letter);

    let number_of_letters = letter.letters.len();

    let num_renders = guess_query.iter().count();

    if num_renders < number_of_letters {
        let letter_value = letter.letters[num_renders].clone();
        let letter_sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                ((num_renders) as f32) * MULTIPLIER,
                ((0) as f32) * MULTIPLIER,
                3.,
            ))
            .with_scale(SCALE),
            texture: resource.texture.clone(),
            ..default()
        };
        let letter_index = letter_map.map.get(letter_value.as_str()).unwrap().clone() as usize;

        let letter_texture = TextureAtlas {
            layout: resource.layout.clone(),
            index: letter_index,
        };

        let id = commands
            .spawn((letter_sprite, letter_texture, GuessLayer))
            .id();
        letter.spawned_images.push(id);
    }
    if num_renders > number_of_letters && !letter.spawned_images.is_empty() {
        commands
            .entity(letter.spawned_images.pop().unwrap())
            .despawn();
    }

    // if number_of_letters as i8 > letter.previous_index {
    //     let letters_sprite = SpriteBundle {
    //         transform: Transform::from_translation(Vec3::new(
    //             ((number_of_letters - 1) as f32) * MULTIPLIER,
    //             ((0) as f32) * MULTIPLIER,
    //             0.,
    //         ))
    //         .with_scale(SCALE),
    //         texture: resource.texture.clone(),
    //         ..default()
    //     };
    //     let letter_index = letter_map.map.get(letter_value.as_str()).unwrap().clone() as usize;

    //     let solved_texture = TextureAtlas {
    //         layout: resource.layout.clone(),
    //         index: letter_index,
    //     };
    // }
}
