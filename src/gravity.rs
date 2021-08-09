use bevy::{
    prelude::*,
};

use crate::{WinSize, Falling};

pub struct GravityPlugin;

impl Plugin for GravityPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_system(gravity_system.system());
    }
}

fn gravity_system(
    mut commands: Commands,
    time: Res<Time>,
    winsize: Res<WinSize>,
    mut query: Query<(Entity, &mut Transform, &mut Falling)>,
){
    let gravity = 300.0 * Vec3::new(0.0, -0.2, 0.0);
    //gravity effects here
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    for (falling_entity, mut transform, mut falling) in query.iter_mut(){
        if falling.gravity_on{
            transform.translation += falling.velocity * delta_seconds;
            falling.velocity = falling.velocity + (gravity * delta_seconds);

            if transform.translation.y < -(winsize.h / 2.0){
                commands.entity(falling_entity).despawn();
            } 
        }
    }
}
