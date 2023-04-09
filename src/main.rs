use ::bevy::prelude::*;
use rand::Rng;

const NUM_OBJECTS: isize = 50;
const OBJ_COLOR: Color = Color::rgb(217. / 255., 32. / 255., 77. / 255.);
const OBJ_SIZE: Vec3 = Vec3::new(20., 20., 0.);

const BG_COLOR: Color = Color::rgb(0., 0., 0.);

const X_MIN: f32 = -300.;
const X_MAX: f32 = 300.;
const Y_MIN: f32 = -300.;
const Y_MAX: f32 = 300.;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_objects)
        .add_systems((handle_direction_pressed,).in_schedule(CoreSchedule::FixedUpdate))
        .run()
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // I was going to draw my triangle, but instead I'll eventually just render a sprite there.
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(shape::
    //  });
}

#[derive(Component)]
struct Obj;

#[derive(Component)]
struct Panel;

fn spawn_objects(mut commands: Commands) {
    println!("MY HELLO");
    for i in 0..NUM_OBJECTS {
        let x = rand::thread_rng().gen_range(X_MIN..X_MAX);
        let y = rand::thread_rng().gen_range(Y_MIN..Y_MAX);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: OBJ_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: OBJ_SIZE,
                    ..default()
                },
                ..default()
            },
            Obj,
        ));
    }
}

fn handle_direction_pressed(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0., Y_MAX / 2., 0.),
                    scale: Vec3::new(X_MAX - X_MIN, (Y_MAX - Y_MIN) / 2., 0.),
                    ..default()
                },
                ..default()
            },
            Panel,
        ));
    }
    if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S){
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 1., 1.),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0., Y_MIN / 2., 0.),
                    scale: Vec3::new(X_MAX - X_MIN, (Y_MAX - Y_MIN) / 2., 0.),
                    ..default()
                },
                ..default()
            },
            Panel,
        ));
    }
    if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A){
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 1., 1.),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(X_MIN / 2., 0., 0.),
                    scale: Vec3::new((X_MAX - X_MIN) / 2., Y_MAX - Y_MIN, 0.),
                    ..default()
                },
                ..default()
            },
            Panel,
        ));
    }
    if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D){
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(X_MAX / 2., 0., 0.),
                    scale: Vec3::new((X_MAX - X_MIN) / 2., Y_MAX - Y_MIN, 0.),
                    ..default()
                },
                ..default()
            },
            Panel,
        ));
    }
}
