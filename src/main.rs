use bevy::prelude::*;

mod player;
mod poles;
use player::*;
use poles::*;

#[derive(Resource)]
struct GameState {
    is_running: bool,
}

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
        .insert_resource(GameState { is_running: true })
        .add_systems(Startup, setup)
        .add_systems(Update, has_lost)
        .add_systems(Update, character_movement)
        .add_systems(FixedUpdate, pole_movement)
        .add_systems(FixedUpdate, (spawn_poles, despawn_poles))
        .run();
}

#[derive(Component)]
pub struct Ground {}

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

fn has_lost(
    mut the_game_state: ResMut<GameState>,
    player: Query<(&Transform, &Player)>,
    colliders: Query<(&Collider, &Transform, &Sprite)>,
) /* -> bool */
{
    for (transform, _) in &player {
        // Check if the player has hit the ground
        if transform.translation.y < -240.0 {
            // println!("Game Over, flew too low");
            the_game_state.is_running = false;
            /* return true; */
        }
    }
    if collider_checks(colliders, player) {
        // println!("Game Over, hit a pole");
        the_game_state.is_running = false;
        /* return true; */
    }
    /* false */
}

fn collider_checks(
    mut colliders: Query<(&Collider, &Transform, &Sprite)>,
    player: Query<(&Transform, &Player)>,
) -> bool {
    for (_, transform, sprite) in colliders.iter_mut() {
        let pos = transform.translation;
        let size = sprite.custom_size.unwrap();
        for (player_transform, _) in player.iter() {
            let player_pos = player_transform.translation; // Declare player_pos here
            let player_size = Vec2::new(100.0, 100.0); // Assuming the size is 100px by 100px

            if player_pos.x + player_size.x / 2.0 > pos.x - size.x / 2.0
                && player_pos.x - player_size.x / 2.0 < pos.x + size.x / 2.0
                && player_pos.y + player_size.y / 2.0 > pos.y - size.y / 2.0
                && player_pos.y - player_size.y / 2.0 < pos.y + size.y / 2.0
            {
                return true;
            }
        }
    }
    false
}
