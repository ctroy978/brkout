use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{Materials, WinSize, Collider, Gravity};

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
    mut query: Query<(Entity, &mut Transform), With<Gravity>>,
){
    let gravity = 300.0;
    //gravity effects here
    for (gravity_entity, mut transform) in query.iter_mut(){
        let translation = &mut transform.translation;
        translation.y  -= gravity * time.delta_seconds();

        if translation.y < -(winsize.h / 2.0){
            commands.entity(gravity_entity).despawn();
        } 
    }
}