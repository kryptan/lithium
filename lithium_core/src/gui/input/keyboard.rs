use super::ButtonState;

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Keyboard {
    just_pressed: Vec<Key>,
    pressed: Vec<Key>,
    just_released: Vec<Key>,

    input: String,
}

impl Keyboard {
    pub fn advance(&mut self) {
        self.just_released.clear();
        self.pressed.extend_from_slice(&self.just_pressed);
        self.just_pressed.clear();
        self.input.clear();
    }

    pub fn press(&mut self, key: Key) {
        self.just_pressed.push(key);
    }

    pub fn release(&mut self, key: Key) {
        self.just_released.push(key);

        self.pressed.retain(|&k| k != key);
        self.just_pressed.retain(|&k| k != key);
    }

    pub fn enter_char(&mut self, char: char) {
        self.input.push(char);
    }

    pub fn enter_str(&mut self, str: &str) {
        self.input += str;
    }

    pub fn key_state(&self, key: Key) -> ButtonState {
        if self.pressed.contains(&key) {
            ButtonState::Pressed
        } else if self.just_pressed.contains(&key) {
            ButtonState::JustPressed
        } else if self.just_released.contains(&key) {
            ButtonState::JustReleased
        } else {
            ButtonState::Released
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        let state = self.key_state(key);
        state == ButtonState::Pressed || state == ButtonState::JustPressed
    }

    pub fn all_pressed(&self) -> &[Key] {
        &self.pressed
    }

    pub fn all_just_pressed(&self) -> &[Key] {
        &self.just_pressed
    }

    pub fn all_just_released(&self) -> &[Key] {
        &self.just_released
    }
}

// from https://github.com/tomaka/winit/blob/master/src/events.rs
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Key {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    // TODO: rename
    Backspace,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LMenu,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward, // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RMenu,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
}