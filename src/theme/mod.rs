#![allow(dead_code)]

pub mod interaction;
mod navigation;
pub mod palette;
pub mod widget;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin, navigation::plugin));
}
