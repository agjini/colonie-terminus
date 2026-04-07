use bevy::prelude::*;

mod damage_popup;
mod progress_bar;
mod xp;

pub use progress_bar::ProgressBar;
pub use progress_bar::progress_bar;

pub use damage_popup::spawn_damage_popup;

pub fn plugin(app: &mut App) {
    app.add_plugins((progress_bar::plugin, damage_popup::plugin, xp::plugin));
}
