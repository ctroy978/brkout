use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

mod ball;
mod block;
mod falling;

use ball::BallPlugin;
use block::BlockPlugin;
use falling::FallingPlugin;

const WALL_THICK: f32 = 10.0;
const BLK_HEIGHT: f32 = 20.0;
const BLK_WIDTH: f32 = 40.0;


struct Materials{
    ball: Handle<ColorMaterial>,
    wall: Handle<ColorMaterial>,
    red_block: Handle<ColorMaterial>,
    blue_block: Handle<ColorMaterial>,
}

struct WinSize{
    w: f32,
    h: f32,
}


struct Ball{
    velocity: Vec3,
}



struct FallingToSpawn(Vec3);

#[derive(PartialEq)]
enum Collider{
    Solid,
    Break,
    Falling,
}


fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: String::from("bouncing ball"),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "walls",
            SystemStage::single(spawn_walls.system()),
            )
        .add_plugin(BallPlugin)
        .add_plugin(BlockPlugin)
        .add_plugin(FallingPlugin)
        .add_system(ball_collision_system.system())
        .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    ){

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let window = windows.get_primary_mut().unwrap();

    commands.insert_resource(WinSize{
        h: window.height(),
        w: window.width(),
    });

    commands.insert_resource(Materials{
        ball: materials.add(Color::rgb(0.0, 0.0, 0.7).into()),
        wall: materials.add(Color::rgb(0.9, 0.9, 0.0).into()),
        red_block: materials.add(Color::rgb(0.9, 0.0, 0.0).into()),
        blue_block: materials.add(Color::rgb(0.0, 0.0, 0.9).into()),

            
    });

}

fn spawn_walls(
    mut commands: Commands,
    materials: ResMut<Materials>,
    winsize: Res<WinSize>,
    ){

    //left wall
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.wall.clone(),
            transform: Transform::from_xyz(-winsize.w / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(WALL_THICK, winsize.h)),
            ..Default::default()
    })
    .insert(Collider::Solid);

    //right wall
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.wall.clone(),
            transform: Transform::from_xyz(winsize.w / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(WALL_THICK, winsize.h)),
            ..Default::default()
        })
        .insert(Collider::Solid);

    //top wall
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.wall.clone(),
            transform: Transform::from_xyz(0.0, winsize.h / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(winsize.w, WALL_THICK)),
            ..Default::default()
        })
        .insert(Collider::Solid);

    //bot wall
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.wall.clone(),
            transform: Transform::from_xyz(0.0, -winsize.h / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(winsize.w, WALL_THICK)),
            ..Default::default()
        })
        .insert(Collider::Solid);
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

                //if hits block
                if *collider == Collider::Break{
                   commands.entity(collider_entity).despawn(); 
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

                if *collider == Collider::Break{
                    //turn it into falling block with Vec3 position.
                    commands
                        .spawn()
                        .insert(FallingToSpawn(transform.translation.clone()));
                }
            }
        }
    }
}

