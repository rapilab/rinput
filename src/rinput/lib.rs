#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
// #![feature(fn_traits)]
#![warn(missing_docs)]

extern crate rustbox;

pub use editor::Editor;
pub use input::Input;

mod input;
mod keyboard;
mod editor;

