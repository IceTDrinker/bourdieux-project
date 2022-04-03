use bevy::prelude::*;

fn greet() {
    println!("Hello Bourdieux!");
}

fn main() {
    App::new().add_system(greet).run();
}
