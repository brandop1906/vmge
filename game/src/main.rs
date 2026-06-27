use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entity)
        .add_systems(Update, (current_position, move_player))
        .run();
}

pub fn spawn_entity(mut commands: Commands,  asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Name { name: "Player".to_string() }, 
        Movement { speed_x: 20.0, speed_y: 20.0 },  // Add the Movement component with desired speed values.
        PlayerControlled, // Add the PlayerControlled component to mark this entity as player-controlled.
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load(r"town.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn(Name { name: "NPC".to_string() });
    
}
pub fn current_position(query: Query<(&Transform, &Name), With<PlayerControlled>>) {
    for (transform, name) in query.iter() {
        println!("{} is at position ({}, {})", name.name, transform.translation.x, transform.translation.y);
    }
}
#[derive(Component)]
pub struct Name
    {
        pub name: String,
    }

#[derive(Component)]
pub struct Movement
    {
    pub speed_x: f32,
    pub speed_y: f32,
    }
    pub fn move_player(mut query: Query<(&mut Transform, &Movement)>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
        for (mut transform, movement) in query.iter_mut() {
            if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
                transform.translation.x -= movement.speed_x * time.delta_secs();
                println!("Moving left");
            } else if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
                transform.translation.x += movement.speed_x * time.delta_secs();
                println!("Moving right");
            }

            if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
                transform.translation.y += movement.speed_y * time.delta_secs();
                println!("Moving up");
            } else if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
                transform.translation.y -= movement.speed_y * time.delta_secs();
                println!("Moving down");
            }
        }
    }

#[derive(Component)]
pub struct PlayerControlled;

