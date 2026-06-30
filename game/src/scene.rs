use bevy::prelude::*;
use crate::walkmesh::*;
use crate::scripting::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SceneLibrary {
    scenes: HashMap<String, Scene>,
    current_scene: String,
}

#[derive(Event)]
pub struct SceneChangeRequest {
    scene_id: String,
}

pub struct Scene {
    background: String,
    npcs: Vec<NpcDef>,
    walkmeshes: Vec<WalkableMesh>,
    scripts: ScriptLibrary,
    scene_change: Vec<ExitDef>,
    default_player_pos: Vec2,
}

pub struct NpcDef {
    sprite: String,
    name: String,
    position: Vec2,
    field_identity: u8,
    solid: bool,
    
}

pub struct ExitDef {
    target_scene: String,
    player_pos: Vec2,
    trigger_area: Rect,
}

pub fn detection(position_query: Query<&Transform, With<PlayerControlled>>, mut scene_change_event_writer: EventWriter<SceneChangeRequest>, 
    scenes: Res<SceneLibrary>) {
    let player_pos = position_query.single().translation.truncate();
    if let Some(current_scene) = scenes.scenes.get(&scenes.current_scene) {
        for scene_change in &current_scene.scene_change {
            if scene_change.trigger_area.contains(player_pos) {
                scene_change_event_writer.send(SceneChangeRequest { scene_id: scene_change.target_scene.clone() });
            }
        }
    }
}

pub fn handle_scene_change(mut scene_library: ResMut<SceneLibrary>, mut scene_change_reader: EventReader<SceneChangeRequest>) {
    for event in scene_change_reader.iter() {
        if let Some(scene) = scene_library.scenes.get(&event.scene_id) {
            scene_library.current_scene = event.scene_id.clone();
            // Handle scene change logic here, such as loading new assets or resetting game state.
        }
    }
}