use bevy::prelude::*;

use crate::{Ball, Materials};


pub struct BallPlugin;

impl Plugin for BallPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "balls",
                SystemStage::single(ball_spawn.system()),   
                  )
            .add_system(ball_movement_system.system());
    }
}

fn ball_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    ){
    commands.spawn_bundle(SpriteBundle{
        material: materials.ball.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        transform: Transform{
            translation: Vec3::new(50.0, 50.0, 10.0),
            ..Default::default()
        },
        ..Default::default()
   })
    .insert(Ball{
        velocity: Vec3::new(0.5, -0.5, 0.0) * 400.0
    })
    .insert(Timer::from_seconds(0.1, true));
}

fn ball_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Ball)> 
    ){
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for( mut transform, ball) in query.iter_mut(){
        transform.translation += ball.velocity * delta_seconds;
    }
}
    
