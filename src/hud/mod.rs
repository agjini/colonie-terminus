use bevy::prelude::*;

mod progress_bar;

pub use progress_bar::ProgressBar;
pub use progress_bar::progress_bar;

pub fn plugin(app: &mut App) {
    app.add_plugins(progress_bar::plugin);
}
