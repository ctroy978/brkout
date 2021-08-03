use bevy::prelude::*;

use crate::{Collider, Materials, FallingToSpawn, Gravity, BLK_WIDTH, BLK_HEIGHT};

pub struct FallingPlugin;

impl Plugin for FallingPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
        .add_system(falling_to_spawn.system());
    }
}

fn falling_to_spawn(
    mut commands: Commands,
    query: Query<(Entity, &FallingToSpawn)>,
    materials: Res<Materials>,
    ){
    for(falling_entity, falling_to_spawn) in query.iter(){
        commands.spawn_bundle(SpriteBundle{
            material: materials.blue_block.clone(),
            sprite: Sprite::new(Vec2::new(BLK_WIDTH, BLK_HEIGHT)),
            transform: Transform{
                translation: falling_to_spawn.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Gravity)
        .insert(Collider::Debris);
        
        commands.entity(falling_entity).despawn();
    }
}
