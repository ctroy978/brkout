use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{Ball, Materials, Collider, FallingToSpawn};


pub struct BallPlugin;

impl Plugin for BallPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "balls",
                SystemStage::single(ball_spawn.system()),   
                  )
            .add_system(ball_movement_system.system())
            .add_system(ball_collision_system.system());
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
        velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
    });
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

fn ball_collision_system(
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    ){

    if let Ok((mut ball, ball_transform, ball_sprite)) = ball_query.single_mut(){
        let ball_size = ball_sprite.size;
        let velocity = &mut ball.velocity;

        for(collider_entity, collider, transform, sprite) in collider_query.iter(){

            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
                );

            if let Some(collision) = collision{
                println!("{:?}", collision);

                //if hits block
                if *collider == Collider::Break{
                   commands.entity(collider_entity).despawn(); 
                    //turn it into falling block with Vec3 position.
                    commands
                        .spawn()
                        .insert(FallingToSpawn(transform.translation.clone()));
                }


                //reflect ball
                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision{
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                if reflect_x{
                    velocity.x = -velocity.x;
                }

                if reflect_y{
                    velocity.y = -velocity.y;
                }

                if *collider == Collider::Solid{
                    break;
                }
            }
        }
    }
}


    
