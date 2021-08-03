use bevy::{
    prelude::*,
};

mod ball;
mod block;
mod falling;
mod gravity;

use ball::BallPlugin;
use block::BlockPlugin;
use falling::FallingPlugin;
use gravity::GravityPlugin;

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

struct Gravity;

struct FallingToSpawn(Vec3);

#[derive(PartialEq)]
enum Collider{
    Solid,
    Break,
    Debris,
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
        .add_plugin(GravityPlugin)
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
            transform: Transform::from_xyz(0.0, -winsize.h / 2.0, 10.), 
            sprite: Sprite::new(Vec2::new(winsize.w, WALL_THICK)),
            ..Default::default()
        })
        .insert(Collider::Solid);
}