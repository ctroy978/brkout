use bevy::prelude::*;


use crate::{Materials, WinSize, Collider, WALL_THICK};

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
    winsize: Res<WinSize>,
    ){
    let playable_area = winsize.w - ((WALL_THICK) * 2.0); 
    let brick_rows = 4;
    let brick_cols = 7;
    let brick_spacing = 7.0;
    let brick_width = playable_area / brick_cols as f32 - brick_spacing;
    let brick_size = Vec2::new(brick_width, 30.0);
    let brick_offset = Vec3::new(-(playable_area - brick_size.x - brick_spacing) / 
                                 2.0, (brick_size.y + brick_spacing) * brick_rows as f32 , 0.0);
    for row in 0..brick_rows{
        let y_pos = row as f32 * (brick_size.y + brick_spacing);
        for col in 0..brick_cols{
            let brick_pos = Vec3::new(
                col as f32 * (brick_size.x + brick_spacing),
                y_pos,
                10.0,
                 ) + brick_offset; 
            //lay bricks
            commands
                .spawn_bundle(SpriteBundle{
                    material: materials.red_block.clone(),
                    sprite: Sprite::new(brick_size),
                    transform: Transform::from_translation(brick_pos),
                    ..Default::default()
                })
                .insert(Collider::Break);
        }
    }
    

}


