use bevy::prelude::*;
use scripting::*;
use player::*;
mod scripting;
mod walkmesh;
mod player;

fn main() {
    let mut walk_area = walkmesh::WalkableArea::new();
    walk_area.add_walkable_mesh(walkmesh::WalkableMesh::new(-620.0, -150.0, 620.0, -75.0));

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ScriptVM::new(vm::assembler::assemble_scene("SOLID 0,1\nWINDOW 100,50,300,100,0\nMESSAGE 0,0\nMESSAGE 0,1\nWINCLOSE 0\nRET"), 
        vec!["Welcome to Midgar!".to_string(), "The reactor is just ahead.".to_string()]))
        .insert_resource(walk_area)
        .add_systems(Startup, (spawn_entity, run_script))
        .add_systems(Update, (move_player, process_vm_commands, render_text, close_dialog_on_input))
        .run();
}

pub fn spawn_entity(mut commands: Commands,  asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(PlayerImages {
        player_down: asset_server.load(r"Zane_down.png"),
        player_up: asset_server.load(r"Zane_up.png"),
        player_left: asset_server.load(r"Zane_left.png"),
        player_right: asset_server.load(r"Zane_right.png"),
    });
    commands.spawn((
        Sprite::from_image(asset_server.load(r"Zane_down.png")),
        Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(0.5)),
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