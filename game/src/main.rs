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
        Sprite::from_image(asset_server.load(r"Zane.png")),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Name { name: "Player".to_string() }, 
        Movement { speed_x: 200.0, speed_y: 200.0 },  // Add the Movement component with desired speed values.
        PlayerControlled, // Add the PlayerControlled component to mark this entity as player-controlled.
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load(r"town.png"),
            custom_size: Some(Vec2::new(1280.0, 720.0)), // Set the custom size of the sprite
            ..default()
        },
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
            let mut direction = Vec2::ZERO;
            if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
            }
            if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
                direction.x += 1.0;
            }
            if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
            }

            if direction != Vec2::ZERO {
                direction = direction.normalize();
                transform.translation.x += direction.x * movement.speed_x  * time.delta_secs();
                transform.translation.y += direction.y * movement.speed_y  * time.delta_secs();
            }

            transform.translation.x = transform.translation.x.clamp(-600.0, 600.0);
            transform.translation.y = transform.translation.y.clamp(-300.0, 300.0);
        }
    }

#[derive(Component)]
pub struct PlayerControlled;

