use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entity)
        .add_systems(Update, (current_position, move_x))
        .run();
}

#[derive(Component)]
pub struct Position 
    {
        pub x: f32,
        pub y: f32,
    }
    pub fn spawn_entity(mut commands: Commands) {
        commands.spawn((Name { name: "Player".to_string() },Position { x: 0.0, y: 0.0 }, MovementX { speed_x: 0.1 },PlayerControlled));
        commands.spawn((Name { name: "NPC".to_string() },Position { x: 10.0, y: 10.0 }));
        
    }
    pub fn current_position(query: Query<(&mut Position, &Name), With<PlayerControlled>>) {
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
pub struct MovementX
    {
    pub speed_x: f32,
    }
    pub fn move_x(mut query: Query<(&mut Position, &MovementX)>, time: Res<Time>) {
        for (mut position, movement) in query.iter_mut() {
            position.x += movement.speed_x * time.delta_secs(); // Move the entity along the x-axis based on speed and elapsed time.
        }
    }
#[derive(Component)]
pub struct PlayerControlled;

