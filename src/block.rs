use bevy::prelude::*;


use crate::{Materials, Collider, BLK_HEIGHT, BLK_WIDTH};

pub struct BlockPlugin;

impl Plugin for BlockPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "blocks",
                SystemStage::single(block_spawn.system()),   
                  );
    }
}

fn block_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    ){
    commands.spawn_bundle(SpriteBundle{
        material: materials.red_block.clone(),
        sprite: Sprite::new(Vec2::new(BLK_WIDTH, BLK_HEIGHT)),  
        transform: Transform{
            translation: Vec3::new(0.0, 100.0, 10.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Collider::Break);
}
