use std::{f32::consts::PI};

use ::bevy::prelude::*;
use bevy::{
    sprite::{Anchor},
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
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/cardinal.png"),
        transform: Transform {
            scale: Vec3::new(0.15, 0.15, 1.),
            ..default()
        },
        ..default()
    });

    let tex = asset_server.load("images/wing.png");
    commands.spawn(WingBundle::new(
        WingPosition::Left,
        WingDirection::Clockwise,
        tex.clone(),
    ));
    commands.spawn(WingBundle::new(
        WingPosition::Right,
        WingDirection::CounterClockwise,
        tex.clone(),
    ));

    

    commands.spawn(
        TextBundle::from_sections([TextSection::new(
            "0",
            TextStyle {
                font: asset_server.load("fonts/Roboto-Black.ttf"),
                font_size: 60.,
                color: Color::rgb(0.5, 0., 0.),
            },
        )])
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

    commands.spawn(WingTimer(Timer::from_seconds(
        0.75,
        TimerMode::Once,
    )));
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

#[derive(Component, Deref, DerefMut)]
struct WingTimer(Timer);

#[derive(Component)]
struct Wing {
    direction: WingDirection,
    position: WingPosition,
    base_z_rotation: f32,
    is_moving: bool,
}

#[derive(Bundle)]
struct WingBundle {
    sprite_bundle: SpriteBundle,
    wing: Wing,
}

impl WingBundle {
    fn new(position: WingPosition, direction: WingDirection, texture: Handle<Image>) -> WingBundle {
        WingBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    anchor: position.anchor(),
                    flip_x: if position == WingPosition::Left { true } else { false },
                    ..default()
                },
                texture: texture,
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    scale: Vec3::new(0.25, 0.25, 0.),
                    ..default()
                },
                ..default()
            },
            wing: Wing {
                direction: direction,
                position: position,
                base_z_rotation: 0.,
                is_moving: false,
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
    Right,
    Left,
}

impl WingPosition {
    fn anchor(&self) -> Anchor {
        match self {
            WingPosition::Right => Anchor::CenterLeft,
            WingPosition::Left => Anchor::CenterRight,
        }
    }

    fn adjusted_angle(&self, original: f32) -> f32 {
        let a = original
            + match self {
                WingPosition::Right => 0.,
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

fn handle_direction_pressed(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut wing_query: Query<(&mut Transform, &mut Wing)>,
    mut timer_query: Query<&mut WingTimer>,
    asset_server: Res<AssetServer>,
) {
    // Can't change direction if wings are moving
    for (transform, wing) in &wing_query {
        if wing.is_moving {
            return;
        }
    }

    let mut rotation = 0.;
    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
        rotation = 0.;
    }
    else if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S) {
        rotation = PI;
    }
    else if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
        rotation = PI / 2.;
    }
    else if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D) {
        rotation = PI * 3. / 2.;
    }
    else {
        return;
    }

    timer_query.single_mut().reset();
    for (mut transform, mut wing) in &mut wing_query {
        wing.is_moving = true;
        wing.base_z_rotation = rotation;
        transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);
        transform.rotate_z(rotation);
    }
}

fn update_wings(
    time: Res<Time>,
    mut timer_query: Query<&mut WingTimer>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Wing)>,
    mut wave_end_events: EventWriter<WaveEndEvent>,
) {
    let mut wing_timer = timer_query.single_mut();
    let done = wing_timer.tick(time.delta()).just_finished();

    for (entity, mut transform, mut wing) in &mut query {
        if !wing.is_moving {
            continue;
        }

        if wing.direction == WingDirection::Clockwise {
            transform.rotate_z(-WING_SPEED * TIME_STEP);
        } else {
            transform.rotate_z(WING_SPEED * TIME_STEP);
        }

        if done {
            wing.is_moving = false;
            transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);
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
        if !wing.is_moving {
            continue;
        }
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

        spawn_objects(commands);
    }
}

fn update_scoreboard(points: Res<Points>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = points.total.to_string();
}
