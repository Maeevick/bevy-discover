use bevy::{prelude::*, state::commands};

fn main() {
    App::new()
        .add_systems(Startup, add_character)
        .add_systems(Startup, hello_world_system)
        .add_systems(Update, greet_characters)
        .run();
}

fn hello_world_system() {
    println!("\n----------------------");
    println!("  HELLO, BEVY ENGINE!");
    println!("----------------------\n");
 }

 #[derive(Component)]
 struct Character;

 #[derive(Component)]
struct Name(String);


 fn add_character(mut commands: Commands) {
    commands.spawn((Character, Name("Zogg Goldclaw".to_string())));
    commands.spawn((Character, Name("Nibbles The Backscratcher".to_string())));
    commands.spawn((Character, Name("Wizzle \"Professor\" Fumblepotion".to_string())));
    commands.spawn((Character, Name("Grimtwig \"Titanarm\" Wobblefist".to_string())));
    commands.spawn((Character, Name("Skritch Rustyknuckle".to_string())));
 }

 fn greet_characters(query: Query<&Name, With<Character>>) {
    println!("--- The Greenlins! ---");
    for name in &query {
        println!("\thi {}!", name.0);
    }
}
