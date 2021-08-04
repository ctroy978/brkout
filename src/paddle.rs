use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const PADDLE_WIDTH: f32 = 80.0;
const PADDLE_HEIGHT: f32 = 15.0;

use crate::{Falling, Paddle, Materials, Collider, 
            WinSize};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "paddle",
                SystemStage::single(paddle_spawn.system()),   
                  )
            .add_system(paddle_movement_system.system())
            .add_system(paddle_collision_system.system());
    }
}

fn paddle_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    winsize: Res<WinSize>,
    ){
    commands.spawn_bundle(SpriteBundle{
        material: materials.paddle.clone(),
        sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        transform: Transform{
            translation: Vec3::new(0.0, -winsize.h / 2.0 + PADDLE_HEIGHT, 10.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Collider::Solid)
        .insert(Paddle{
            speed: 500.0,
        });
}

fn paddle_movement_system(
    time: Res<Time>,
    winsize: Res<WinSize>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
    ){
    if let Ok((paddle, mut transform)) = query.single_mut(){
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left){
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right){
            direction += 1.0;
        }
        let translation = &mut transform.translation;
        translation.x += time.delta_seconds() * direction * paddle.speed;
        //boundary with walls
        translation.x = translation.x
            .min(
                (winsize.w / 2.0) - (PADDLE_WIDTH / 2.0) - 5.0 
            ).max(
                (-winsize.w / 2.0) + (PADDLE_WIDTH / 2.0) + 5.0
            );
    }
}

fn paddle_collision_system(
    mut collider_query: Query<(Entity, &mut Falling, &Collider, &Transform, &Sprite)>,
    mut paddle_query: Query<(&Paddle, &Transform, &Sprite)>,
    ){
    
    if let Ok((paddle, paddle_transform, sprite)) = paddle_query.single_mut(){
        let paddle_size = sprite.size;

        //check collision with blocks
        for(collider_entity, mut falling, collider, transform, sprite) in collider_query.iter_mut(){

            let collision = collide(
                paddle_transform.translation,
                paddle_size,
                transform.translation,
                sprite.size,
               );

            if let Some(collision) = collision{
                //bounce the object off the paddle
                falling.velocity.y = 100.0;
                if transform.translation.x > paddle_transform.translation.x{
                    falling.velocity.x = 20.0;
                }else{
                    falling.velocity.x = -20.0;
                }
            }
        }
    }
}

