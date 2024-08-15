use super::manual_bindgen::Letter;
use bevy::prelude::*;

pub struct HighlightStartingLetterPlugin;
impl Plugin for HighlightStartingLetterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_highlighter.run_if(does_cursor_exist));
        // app.add_systems(Update, render_cursor);
    }
}

#[derive(Debug, Resource)]
struct PlayerCursor(u8);

fn does_cursor_exist(cursor: Res<PlayerCursor>, query: Query<&Letter>) -> bool {
    cursor.is_added()
}

fn spawn_highlighter(query: Query<&Letter>, mut commands: Commands, cursor: Res<PlayerCursor>) {
    // if !cursor.is_added() {
    //     let number_of_letters = query.iter().count();
    //     if number_of_letters == 5 {
    //         commands.insert_resource(PlayerCursor(0));
    //     }
    // }
    commands.insert_resource(PlayerCursor(0));
}

fn render_cursor(cursor: Res<PlayerCursor>) {
    if cursor.is_added() {
        println!("Cursor is added!");
    }
}
