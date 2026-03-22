use bevy::prelude::*;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Down,
    Left,
    Right,
    Up,
    A,
    B,
    C,
    X,
    Y,
    Z,
    L,
    R,
    Shift,
    Ctrl,
    Alt,
    F5,
    F6,
    F7,
    F8,
    F9,
    MouseLeft,
    MouseMiddle,
    MouseRight,
}

#[derive(Resource)]
pub struct Input {
    pressed_keys: HashSet<Key>,
    previous_pressed_keys: HashSet<Key>,
    triggered_keys: HashSet<Key>,
    repeated_keys: HashSet<Key>,
    repeat_count: HashMap<Key, i32>,
    repeat_start: i32,
    repeat_interval: i32,
    mouse_x: f32,
    mouse_y: f32,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            previous_pressed_keys: HashSet::new(),
            triggered_keys: HashSet::new(),
            repeated_keys: HashSet::new(),
            repeat_count: HashMap::new(),
            repeat_start: 15,
            repeat_interval: 6,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.triggered_keys.clear();
        self.repeated_keys.clear();

        for key in self.pressed_keys.iter() {
            let count = self.repeat_count.entry(*key).or_insert(0);
            *count += 1;

            if *count == 1 {
                self.triggered_keys.insert(*key);
            }

            if *count == 1 || (*count > self.repeat_start && (*count - self.repeat_start) % self.repeat_interval == 0) {
                self.repeated_keys.insert(*key);
            }
        }

        for key in self.previous_pressed_keys.iter() {
            if !self.pressed_keys.contains(key) {
                self.repeat_count.remove(key);
            }
        }

        self.previous_pressed_keys = self.pressed_keys.clone();
    }

    pub fn press(&mut self, key: Key) {
        self.pressed_keys.insert(key);
    }

    pub fn release(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_triggered(&self, key: Key) -> bool {
        self.triggered_keys.contains(&key)
    }

    pub fn is_repeated(&self, key: Key) -> bool {
        self.repeated_keys.contains(&key)
    }

    pub fn dir4(&self) -> i32 {
        if self.is_pressed(Key::Down) {
            return 2;
        }
        if self.is_pressed(Key::Left) {
            return 4;
        }
        if self.is_pressed(Key::Right) {
            return 6;
        }
        if self.is_pressed(Key::Up) {
            return 8;
        }
        0
    }

    pub fn dir8(&self) -> i32 {
        let down = self.is_pressed(Key::Down);
        let left = self.is_pressed(Key::Left);
        let right = self.is_pressed(Key::Right);
        let up = self.is_pressed(Key::Up);

        match (up, down, left, right) {
            (true, false, false, false) => 8,
            (true, false, true, false) => 7,
            (false, false, true, false) => 4,
            (false, true, true, false) => 1,
            (false, true, false, false) => 2,
            (false, true, false, true) => 3,
            (false, false, false, true) => 6,
            (true, false, false, true) => 9,
            _ => 0,
        }
    }

    pub fn set_repeat(&mut self, start: i32, interval: i32) {
        self.repeat_start = start;
        self.repeat_interval = interval;
    }

    pub fn mouse_x(&self) -> i32 {
        self.mouse_x as i32
    }

    pub fn mouse_y(&self) -> i32 {
        self.mouse_y as i32
    }

    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
}

pub fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut input: ResMut<Input>,
    window_query: Query<&Window>,
) {
    input.pressed_keys.clear();

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        input.press(Key::Down);
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        input.press(Key::Left);
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        input.press(Key::Right);
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        input.press(Key::Up);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.press(Key::A);
    }
    if keyboard_input.pressed(KeyCode::KeyB) {
        input.press(Key::B);
    }
    if keyboard_input.pressed(KeyCode::KeyC) {
        input.press(Key::C);
    }
    if keyboard_input.pressed(KeyCode::KeyX) {
        input.press(Key::X);
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        input.press(Key::Y);
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        input.press(Key::Z);
    }
    if keyboard_input.pressed(KeyCode::KeyL) {
        input.press(Key::L);
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        input.press(Key::R);
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        input.press(Key::Shift);
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
        input.press(Key::Ctrl);
    }
    if keyboard_input.pressed(KeyCode::AltLeft) || keyboard_input.pressed(KeyCode::AltRight) {
        input.press(Key::Alt);
    }
    if keyboard_input.pressed(KeyCode::F5) {
        input.press(Key::F5);
    }
    if keyboard_input.pressed(KeyCode::F6) {
        input.press(Key::F6);
    }
    if keyboard_input.pressed(KeyCode::F7) {
        input.press(Key::F7);
    }
    if keyboard_input.pressed(KeyCode::F8) {
        input.press(Key::F8);
    }
    if keyboard_input.pressed(KeyCode::F9) {
        input.press(Key::F9);
    }

    if mouse_button_input.pressed(MouseButton::Left) {
        input.press(Key::MouseLeft);
    }
    if mouse_button_input.pressed(MouseButton::Middle) {
        input.press(Key::MouseMiddle);
    }
    if mouse_button_input.pressed(MouseButton::Right) {
        input.press(Key::MouseRight);
    }

    if let Ok(window) = window_query.get_single() {
        if let Some(cursor_position) = window.cursor_position() {
            input.set_mouse_position(cursor_position.x, window.height() - cursor_position.y);
        }
    }

    input.update();
}

pub fn setup_input(mut commands: Commands) {
    commands.insert_resource(Input::new());
}
