use std::f32::consts::PI;

use ::bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

const TIME_STEP: f32 = 1. / 60.;

const NUM_EGGS: isize = 25;

const BG_COLOR: Color = Color::rgb(0., 0.4, 0.1);

const RADIUS: f32 = 300.;

const WING_SPEED: f32 = 2.;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(GameState { state: State::TimeUp })
        .insert_resource(Points {
            total: 0,
            this_wave: 0,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_event::<WaveEndEvent>()
        .add_systems(
            (
                handle_direction_pressed,
                handle_reset,
                update_wings,
                check_wing_collisions.after(update_wings),
                handle_wave_end.after(check_wing_collisions),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_system(update_timer)
        .add_system(update_scoreboard)
        .add_system(update_time_display)
        .add_system(bevy::window::close_on_esc)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<GameState>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/nest.png"),
        transform: Transform {
            scale: Vec3::new(0.6, 0.6, 1.),
            translation: Vec3::new(0., 0., 0.),
            ..default()
        },
        ..default()
    });

    let bird_tex = asset_server.load("images/cardinal.png");
    commands.spawn(SpriteBundle {
        texture: bird_tex,
        transform: Transform {
            scale: Vec3::new(0.15, 0.15, 50.),
            translation: Vec3::new(0., 0., 50.),
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

    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "0",
            TextStyle {
                font: asset_server.load("fonts/Roboto-Black.ttf"),
                font_size: 60.,
                color: Color::rgb(1., 1., 1.),
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(20.),
                left: Val::Px(200.),
                ..default()
            },
            ..default()
        }),
        ScoreDisplay,
    ));

    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "30",
            TextStyle {
                font: asset_server.load("fonts/Roboto-Black.ttf"),
                font_size: 60.,
                color: Color::rgb(1., 1., 1.),
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(20.),
                right: Val::Px(200.),
                ..default()
            },
            ..default()
        }),
        TimeDisplay,
    ));
    commands.spawn(GameTimer(Timer::from_seconds(15., TimerMode::Once)));
    

    commands.spawn(WingTimer(Timer::from_seconds(0.75, TimerMode::Once)));

    let egg_tex = asset_server.load("images/egg.png");
    spawn_eggs(commands, egg_tex);

    state.state = State::Playing;
}

#[derive(Resource)]
struct Points {
    total: isize,
    this_wave: isize,
}

#[derive(Resource,  Deref, DerefMut)]
struct GameState {
    state: State,
}

#[derive(PartialEq)]
enum State {
    Playing, TimeUp
}

#[derive(Default)]
struct WaveEndEvent;

#[derive(Component)]
struct Egg;

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct TimeDisplay;

#[derive(Component, Deref, DerefMut)]
struct WingTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct GameTimer(Timer);

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
                    flip_x: if position == WingPosition::Left {
                        true
                    } else {
                        false
                    },
                    ..default()
                },
                texture: texture,
                transform: Transform {
                    translation: Vec3::new(0., 0., 49.),
                    scale: Vec3::new(0.2, 0.15, 0.),
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

fn spawn_eggs(mut commands: Commands, texture: Handle<Image>) {
    for i in 0..NUM_EGGS {
        let cardinal_padding = 100.;
        let r = rand::thread_rng().gen_range(cardinal_padding..RADIUS);
        let theta = rand::thread_rng().gen_range(-PI..PI);
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3::new(theta.cos() * r, theta.sin() * r, (2 + i) as f32),
                    scale: Vec3::new(0.1, 0.1, 1.),
                    ..default()
                },
                ..default()
            },
            Egg,
        ));
    }
}

fn handle_direction_pressed(
    keyboard_input: Res<Input<KeyCode>>,
    mut wing_query: Query<(&mut Transform, &mut Wing)>,
    mut timer_query: Query<&mut WingTimer>,
    game_state: Res<GameState>,
) {
    if game_state.state == State::TimeUp {
        return;
    }

    // Can't change direction if wings are moving
    for (_, wing) in &wing_query {
        if wing.is_moving {
            return;
        }
    }

    let rotation;
    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
        rotation = 0.;
    } else if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S)
    {
        rotation = PI;
    } else if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A)
    {
        rotation = PI / 2.;
    } else if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D)
    {
        rotation = PI * 3. / 2.;
    } else {
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

fn handle_reset(
    keyboard_input: Res<Input<KeyCode>>,
    mut timer_query: Query<&mut GameTimer>,
    mut game_state: ResMut<GameState>,
    mut points: ResMut<Points>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
    timer_query.single_mut().reset();
    game_state.state = State::Playing;
    points.this_wave = 0;
    points.total = 0;
}

fn update_wings(
    time: Res<Time>,
    mut timer_query: Query<&mut WingTimer>,
    mut query: Query<(&mut Transform, &mut Wing)>,
    mut wave_end_events: EventWriter<WaveEndEvent>,
) {
    let mut wing_timer = timer_query.single_mut();
    let done = wing_timer.tick(time.delta()).just_finished();

    for (mut transform, mut wing) in &mut query {
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
    objs_query: Query<(Entity, &Transform), With<Egg>>,
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
    objs_query: Query<Entity, With<Egg>>,
    mut points: ResMut<Points>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>
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

        let tex = asset_server.load("images/egg.png");

        if game_state.state != State::TimeUp {
            spawn_eggs(commands, tex);
        }
    }
}

fn update_scoreboard(points: Res<Points>, mut query: Query<&mut Text, With<ScoreDisplay>>) {
    let mut text = query.single_mut();
    text.sections[0].value = points.total.to_string();
}

fn update_timer(time: Res<Time>, mut timer_query: Query<&mut GameTimer>, mut game_state: ResMut<GameState>) {
    let mut timer = timer_query.single_mut();
    if timer.tick(time.delta()).just_finished() {
        game_state.state = State::TimeUp;
    }
}

fn update_time_display(
    mut timer_query: Query<&mut GameTimer>,
    mut query: Query<&mut Text, With<TimeDisplay>>,
) {
    let timer = timer_query.single_mut();

    let mut text = query.single_mut();
    text.sections[0].value = timer.remaining_secs().ceil().to_string();
}
