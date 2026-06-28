use bevy::prelude::*;

#[derive(Resource)]
struct ScriptVM {
    vm: vm::interpreter::VM,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ScriptVM { vm: vm::interpreter::VM::new(vm::assembler::assemble_scene("SOLID 1\nWINDOW 100,50,300,100,0\nMESSAGE 0,0\nRET")) }) // Initialize the ScriptVM resource with a new VM instance.
        .add_systems(Startup, (spawn_entity, run_script))
        .add_systems(Update, (move_player, process_vm_commands, render_text, close_dialog_on_input))
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
        Solid, // Add the Solid component to mark this entity as solid. This can be used for collision detection or other purposes.
        FieldEntityId { id: 0 } // Add the FieldEntityId component with an ID of 0. This can be used to identify entities within a game field or similar structure.
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

#[derive(Component)]
pub struct Solid; // Add a Solid component to represent solid entities.

#[derive(Component)]
pub struct FieldEntityId {
    pub id: u8,
}

#[derive(Component)]
pub struct WindowId(pub u8);

#[derive(Component)]
pub struct TextContent(pub String);

fn run_script(mut script: ResMut<ScriptVM>) {
    script.vm.run();
}

fn process_vm_commands(mut script: ResMut<ScriptVM>, query_set_solid: Query<(Entity, &FieldEntityId)>, 
    query_window_close: Query<(Entity, &WindowId)>, mut commands: Commands) 
    {
    let mut unprocessed = Vec::new();
    for command in script.vm.commands.drain(..) {
        match command {
            vm::commands::Command::SetSolid { character_id, enabled } => {
                for (entity, field_entity_id) in query_set_solid.iter() {
                    if field_entity_id.id == character_id {
                        if enabled {
                            commands.entity(entity).insert(Solid);
                        } else {
                            commands.entity(entity).remove::<Solid>();
                        }
                    }
                }
            }
            vm::commands::Command::WindowOpen { x, y, width, height, window_id } => {
                commands.spawn((
                    Transform::from_xyz(x as f32, y as f32, 2.0),
                    Sprite {
                        color: Color::srgba(0.0, 0.1, 0.3, 0.9),
                        custom_size: Some(Vec2::new(width as f32, height as f32)),
                        ..default()
                    },
                    WindowId(window_id),
                ));
            }

            vm::commands::Command::Message { window_id, text } => {
                let mut found = false;
                for (entity, id) in query_window_close.iter() {
                    if id.0 == window_id {
                        commands.entity(entity).insert(TextContent(text.clone()));
                        found = true;
                    }
                }
                if !found {
                    unprocessed.push(vm::commands::Command::Message { window_id, text });
                }
            }

            vm::commands::Command::WindowClose { window_id } => {
                for (entity, window_id_component) in query_window_close.iter() {
                    if window_id_component.0 == window_id {
                        commands.entity(entity).despawn()
                    }
                }
            }
            _ => {}
        }
    }
    script.vm.commands.extend(unprocessed);
}

fn render_text(query: Query<(Entity, &TextContent), Added<TextContent>>, mut commands: Commands) {
    for (entity, text_content) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Text2d::new(&text_content.0),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
            ));
        });
    }
}

fn close_dialog_on_input(
    inputs: Res<ButtonInput<KeyCode>>, 
    query: Query<Entity, With<WindowId>>,
    mut commands: Commands,
){
    if inputs.just_pressed(KeyCode::Space) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
