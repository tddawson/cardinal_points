use ::bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

const TIME_STEP: f32 = 1. / 60.;

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
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_objects)
        .add_systems(
            (handle_direction_pressed, update_wings).in_schedule(CoreSchedule::FixedUpdate),
        )
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
struct Wing {
    direction: WingDirection,
}

#[derive(Bundle)]
struct WingBundle {
    sprite_bundle: SpriteBundle,
    wing: Wing,
}

impl WingBundle {
    fn new(position: WingPosition, direction: WingDirection) -> WingBundle {
        WingBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    anchor: position.anchor(),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    scale: position.size(),
                    ..default()
                },
                ..default()
            },
            wing: Wing {
                direction: direction,
            },
        }
    }
}

#[derive(PartialEq)]
enum WingDirection {
    Clockwise,
    CounterClockwise,
}

enum WingPosition {
    Top,
    Right,
    Bottom,
    Left,
}

impl WingPosition {
    fn anchor(&self) -> Anchor {
        match self {
            WingPosition::Top => Anchor::BottomCenter,
            WingPosition::Right => Anchor::CenterLeft,
            WingPosition::Bottom => Anchor::TopCenter,
            WingPosition::Left => Anchor::CenterRight,
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            WingPosition::Top | WingPosition::Bottom  => Vec3::new(20., (Y_MAX - Y_MIN) / 2., 0.),
            WingPosition::Right | WingPosition::Left => Vec3::new((X_MAX - X_MIN) / 2., 20., 0.)
        }
    }
}

fn spawn_objects(mut commands: Commands) {
    for _ in 0..NUM_OBJECTS {
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
        commands.spawn(WingBundle::new(WingPosition::Left, WingDirection::Clockwise));
        commands.spawn(WingBundle::new(WingPosition::Right, WingDirection::CounterClockwise));
    }
    if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S) {
        commands.spawn(WingBundle::new(WingPosition::Left, WingDirection::CounterClockwise));
        commands.spawn(WingBundle::new(WingPosition::Right, WingDirection::Clockwise));
    }
    if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
        commands.spawn(WingBundle::new(WingPosition::Top, WingDirection::CounterClockwise));
        commands.spawn(WingBundle::new(WingPosition::Bottom, WingDirection::Clockwise));
    }
    if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D) {
        commands.spawn(WingBundle::new(WingPosition::Top, WingDirection::Clockwise));
        commands.spawn(WingBundle::new(WingPosition::Bottom, WingDirection::CounterClockwise));
    }
}

fn update_wings(mut query: Query<(&mut Transform, &Wing)>) {
    for (mut transform, wing) in &mut query {
        if wing.direction == WingDirection::Clockwise {
            transform.rotate_z(-1.0 * TIME_STEP);
        } else {
            transform.rotate_z(1.0 * TIME_STEP);
        }
        
    }
}
