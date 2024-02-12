use bevy::prelude::*;

mod poles;
use poles::*;

const GRAVITY: f32 = 300.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (has_lost, pole_movement))
        .add_systems(FixedUpdate, (spawn_poles, despawn_poles))
        .add_systems(Update, character_movement)
        .run();
}



#[derive(Component)]
pub struct Ground {}
#[derive(Component)]
pub struct Player {
    speed: f32,
    jump_index: u8,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // setup camera
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    // setup ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(480.0, 10.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -240.0, 0.0),
                ..default()
            },
            ..default()
        },
        Ground {},
    ));

    // setup player
    let texture = asset_server.load("bird.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player {
            speed: 225.0,
            jump_index: 0,
        },
    ));
}


fn character_movement(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        let gravity = GRAVITY * time.delta_seconds();
        if player.jump_index > 0 {
            transform.translation.y += movement_amount;
            player.jump_index += 1;
        } else if input.pressed(KeyCode::Space) {
            transform.translation.y += movement_amount;
            player.jump_index += 1;
        }
        if player.jump_index >= 40 {
            player.jump_index = 0;
        }
        if player.jump_index == 0 {
            transform.translation.y -= gravity;
        }
    }
}

fn has_lost(
	player: Query<(&Transform, &Player)>,
	colliders: Query<(&Collider, &Transform, &Sprite)>,
) {
    for (transform, _) in &player {
        // Check if the player has hit the ground
        if transform.translation.y < -240.0 {
			println!("Game Over, flew too low")
        }
	}
	if collider_checks(colliders, player) {
		println!("Game Over, hit a pole")
	}
}

fn collider_checks (
    mut colliders: Query<(&Collider, &Transform, &Sprite)>,
    player: Query<(&Transform, &Player)>,
) -> bool {
    for (_, transform, sprite) in colliders.iter_mut() {
        let pos = transform.translation;
		let size = sprite.custom_size.unwrap();
		for (player_transform, _) in &player {
			let player_pos = player_transform.translation;
			if player_pos.x < pos.x + size.x / 2.0
				&& player_pos.x > pos.x - size.x / 2.0
				&& player_pos.y < pos.y + size.y / 2.0
				&& player_pos.y > pos.y - size.y / 2.0
			{
				return true;
			}
		}
    }
	false
}
