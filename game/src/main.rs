use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entity)
        .add_systems(Update, current_position)
        .run();
}

#[derive(Component)]
pub struct Position 
    {
        pub x: f32,
        pub y: f32,
    }
    pub fn spawn_entity(mut commands: Commands) {
        commands.spawn((Name { name: "Player".to_string() },Position { x: 0.0, y: 0.0 }, PlayerControlled));
        commands.spawn((Name { name: "NPC".to_string() },Position { x: 10.0, y: 10.0 }));
        
    }
    pub fn current_position(query: Query<(&Position, &Name), With<PlayerControlled>>) {
        for (position, name) in query.iter() {
            println!("{} is at position ({}, {})", name.name, position.x, position.y);
        }
    }
#[derive(Component)]
pub struct Name
    {
        pub name: String,
    }

#[derive(Component)]
pub struct PlayerControlled;

