#![warn(missing_docs)]

pub mod input;
pub mod color;
pub mod tone;
pub mod table;
pub mod rect;
pub mod types;
pub mod sharedstate;
pub mod graphics;
pub mod audio;
pub mod config;

pub use color::Color;
pub use tone::Tone;
pub use rect::Rect;
pub use table::Table;
pub use sharedstate::{SharedState, RgssVersion, setup_shared_state, update_shared_state};
pub use graphics::{Graphics, setup_graphics, update_graphics};
pub use audio::{Audio, Bgm, Bgs, Me, Se, PlayState, setup_audio};
pub use config::{Config, WindowMode, setup_config, load_config_from_json, save_config_to_json};
