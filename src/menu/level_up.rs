use bevy::prelude::*;

use crate::gameplay::player::Xp;
use crate::{menu::Menu, theme::widget};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::LevelUp), spawn_level_up_menu);
}

fn spawn_level_up_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("LevelUp Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header("Level up"),
            widget::button("Skip", skip_menu),
        ],
    ));
}

fn skip_menu(
    _: On<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut xp: Single<&mut Xp>,
) {
    xp.skip();
    next_menu.set(Menu::None);
}
