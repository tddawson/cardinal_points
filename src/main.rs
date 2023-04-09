use std::{f32::consts::PI, thread::spawn};

use ::bevy::prelude::*;
use bevy::{
    a11y::accesskit::Point,
    sprite::{Anchor, MaterialMesh2dBundle},
};
use rand::Rng;

const TIME_STEP: f32 = 1. / 60.;

const NUM_OBJECTS: isize = 20;
const OBJ_COLOR: Color = Color::rgb(217. / 255., 32. / 255., 77. / 255.);
const OBJ_SIZE: Vec3 = Vec3::new(40., 40., 0.);

const BG_COLOR: Color = Color::rgb(0., 0., 0.);

const X_MIN: f32 = -300.;
const X_MAX: f32 = 300.;
const Y_MIN: f32 = -300.;
const Y_MAX: f32 = 300.;

const WING_SPEED: f32 = 2.;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(Points {
            total: 0,
            this_wave: 0,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_objects)
        .add_event::<WaveEndEvent>()
        .add_systems(
            (
                handle_direction_pressed,
                update_wings,
                check_wing_collisions.after(update_wings),
                handle_wave_end.after(check_wing_collisions),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_system(update_scoreboard)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(1., 1., 1.))),
        ..default()
    });

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Black.ttf"),
                    font_size: 60.,
                    color: Color::rgb(0.5, 0., 0.),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Percent(0.5),
                left: Val::Percent(0.5),
                ..default()
            },
            ..default()
        }),
    );
}

#[derive(Resource)]
struct Points {
    total: isize,
    this_wave: isize,
}

#[derive(Default)]
struct WaveEndEvent;

#[derive(Component)]
struct Obj;

#[derive(Component)]
struct Wing {
    direction: WingDirection,
    position: WingPosition,
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
                position: position,
            },
        }
    }
}

#[derive(PartialEq)]
enum WingDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(PartialEq)]
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
            WingPosition::Top | WingPosition::Bottom => Vec3::new(20., (Y_MAX - Y_MIN) / 2., 0.),
            WingPosition::Right | WingPosition::Left => Vec3::new((X_MAX - X_MIN) / 2., 20., 0.),
        }
    }

    fn adjusted_angle(&self, original: f32) -> f32 {
        let a = original
            + match self {
                WingPosition::Top => PI / 2.,
                WingPosition::Right => 0.,
                WingPosition::Bottom => 3. * PI / 2.,
                WingPosition::Left => PI,
            };
        let a = a + 2. * PI;
        a % (2. * PI)
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

fn handle_direction_pressed(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, wing_query: Query<&Wing>) {
    // Once wings are out, they've gotta close before you can spawn new ones!
    if wing_query.iter().count() > 0 {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
        commands.spawn(WingBundle::new(
            WingPosition::Left,
            WingDirection::Clockwise,
        ));
        commands.spawn(WingBundle::new(
            WingPosition::Right,
            WingDirection::CounterClockwise,
        ));
    }
    if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S) {
        commands.spawn(WingBundle::new(
            WingPosition::Left,
            WingDirection::CounterClockwise,
        ));
        commands.spawn(WingBundle::new(
            WingPosition::Right,
            WingDirection::Clockwise,
        ));
    }
    if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
        commands.spawn(WingBundle::new(
            WingPosition::Top,
            WingDirection::CounterClockwise,
        ));
        commands.spawn(WingBundle::new(
            WingPosition::Bottom,
            WingDirection::Clockwise,
        ));
    }
    if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D) {
        commands.spawn(WingBundle::new(WingPosition::Top, WingDirection::Clockwise));
        commands.spawn(WingBundle::new(
            WingPosition::Bottom,
            WingDirection::CounterClockwise,
        ));
    }
}

fn update_wings(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Wing)>,
    mut wave_end_events: EventWriter<WaveEndEvent>,
) {
    for (entity, mut transform, wing) in &mut query {
        if wing.direction == WingDirection::Clockwise {
            transform.rotate_z(-WING_SPEED * TIME_STEP);
        } else {
            transform.rotate_z(WING_SPEED * TIME_STEP);
        }

        if transform.rotation.to_euler(EulerRot::XYZ).2.abs() >= PI / 2. {
            commands.entity(entity).despawn();
            wave_end_events.send_default();
        }
    }
}

fn check_wing_collisions(
    mut commands: Commands,
    wings_query: Query<(&Transform, &Wing)>,
    objs_query: Query<(Entity, &Transform), With<Obj>>,
    mut points: ResMut<Points>,
) {
    for (wing_transform, wing) in &wings_query {
        let angle = wing_transform.rotation.to_euler(EulerRot::XYZ).2;
        let wing_angle = wing.position.adjusted_angle(angle);

        for (entity, obj_transform) in &objs_query {
            let mut obj_angle = obj_transform
                .translation
                .y
                .atan2(obj_transform.translation.x);
            obj_angle += 2. * PI;
            let obj_angle = obj_angle % (2. * PI);
            if (obj_angle - wing_angle).abs() < 0.05 {
                commands.entity(entity).despawn();
                points.this_wave = points.this_wave + 1;
            }
        }
    }
}

fn handle_wave_end(
    mut commands: Commands,
    mut wave_end_events: EventReader<WaveEndEvent>,
    objs_query: Query<Entity, With<Obj>>,
    mut points: ResMut<Points>,
) {
    if !wave_end_events.is_empty() {
        wave_end_events.clear();
        let mut num_remaining_objs = 0;
        for entity in &objs_query {
            num_remaining_objs += 1;
            commands.entity(entity).despawn();
        }
        points.total = points.total + points.this_wave - num_remaining_objs;
        points.this_wave = 0;
        println!("{}", points.total);

        spawn_objects(commands);
    }
}

fn update_scoreboard(points: Res<Points>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = points.total.to_string();
}