use bevy::prelude::*;

mod damage_popup;
pub mod elimination;
pub mod panel;
mod player;
mod progress_bar;
pub mod timer;
mod weapon;

pub use damage_popup::spawn_damage_popup;
pub use progress_bar::ProgressBar;
pub use progress_bar::progress_bar;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        progress_bar::plugin,
        damage_popup::plugin,
        player::plugin,
        weapon::plugin,
        timer::plugin,
        elimination::plugin,
    ));
}
