use bevy::prelude::*;
use rpg_hacker_rgss::input::{self, Input};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RPG Hacker - RGSS".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, input::setup_input))
        .add_systems(Update, (rotate_sprite, input::input_system, update_input_display))
        .run();
}

#[derive(Component)]
struct RotatingSprite;

#[derive(Component)]
struct InputDisplay;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.5, 0.75),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RotatingSprite,
    ));

    commands.spawn((
        TextBundle::from_section(
            "RPG Hacker - RGSS Backend",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Input State:\n",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 0.0),
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        InputDisplay,
    ));
}

fn rotate_sprite(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingSprite>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds() * 0.5);
    }
}

fn update_input_display(input: Res<Input>, mut query: Query<&mut Text, With<InputDisplay>>) {
    for mut text in &mut query {
        let mut state = String::new();
        
        if input.is_pressed(input::Key::Up) {
            state.push_str("↑ ");
        }
        if input.is_pressed(input::Key::Down) {
            state.push_str("↓ ");
        }
        if input.is_pressed(input::Key::Left) {
            state.push_str("← ");
        }
        if input.is_pressed(input::Key::Right) {
            state.push_str("→ ");
        }
        
        if input.is_pressed(input::Key::A) {
            state.push_str("A ");
        }
        if input.is_pressed(input::Key::B) {
            state.push_str("B ");
        }
        if input.is_pressed(input::Key::C) {
            state.push_str("C ");
        }
        if input.is_pressed(input::Key::X) {
            state.push_str("X ");
        }
        if input.is_pressed(input::Key::Y) {
            state.push_str("Y ");
        }
        if input.is_pressed(input::Key::Z) {
            state.push_str("Z ");
        }
        
        state.push_str(&format!("\nDir4: {}, Dir8: {}", input.dir4(), input.dir8()));
        state.push_str(&format!("\nMouse: ({}, {})", input.mouse_x(), input.mouse_y()));
        
        text.sections[1].value = state;
    }
}
