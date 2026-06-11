use bevy::prelude::*;

use crate::gameplay::player::{LevelUp, Xp};
use crate::menu::Menu;
use crate::theme::widget;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::LevelUp), spawn_level_up_menu);
    app.add_observer(open_menu);
}

fn open_menu(_on: On<LevelUp>, mut next: ResMut<NextState<Menu>>) {
    next.set(Menu::LevelUp);
}

fn spawn_level_up_menu(mut commands: Commands, xp: Single<&Xp>) {
    commands.spawn((
        widget::ui_root("LevelUp Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::LevelUp),
        children![
            widget::header(format!("Level up {}", xp.level)),
            widget::button("Attack +", attack_up),
            widget::button("Attack speed +", attack_up),
            widget::button("Health max +", attack_up),
            widget::button("Skip", skip_menu),
        ],
    ));
}

fn attack_up(_: On<Pointer<Click>>, mut next: ResMut<NextState<Menu>>, mut xp: Single<&mut Xp>) {
    xp.level_up();
    next.set(Menu::None);
}

fn skip_menu(_: On<Pointer<Click>>, mut next: ResMut<NextState<Menu>>, mut xp: Single<&mut Xp>) {
    xp.skip();
    next.set(Menu::None);
}
