use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {}

pub fn pole_movement(mut poles: Query<(&mut Transform, &mut Collider)>, time: Res<Time>) {
    for (mut transform, _) in &mut poles {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }
}

pub fn spawn_poles(
    mut commands: Commands,
    time: Res<Time>,
    //setup colliders
) {
    if time.elapsed_seconds_f64() % 2.0 == 0.0 {
        let y_position = if rand::random() { 120.0 } else { -120.0 };

        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(30.0, 240.0)), 
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(320.0, y_position, 0.0),
                ..default()
            },
            ..default()
        }, Collider {}));
    }
}

pub fn despawn_poles(mut commands: Commands, poles: Query<(Entity, &Transform), With<Collider>>) {
    for (entity, transform) in &poles {
        if transform.translation.x < -320.0 {
            commands.entity(entity).despawn();
        }
    }
}