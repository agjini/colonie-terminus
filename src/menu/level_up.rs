use crate::gameplay::health::Health;
use crate::gameplay::player::weapon::WeaponSlots;
use crate::gameplay::player::{LevelUp, Player, Xp};
use crate::menu::{Menu, MenuAssets};
use crate::theme::widget;
use bevy::prelude::*;
use widget::{button, header, ui_root};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::LevelUp), spawn_level_up_menu);
    app.add_observer(open_menu);
}

fn open_menu(_on: On<LevelUp>, mut next: ResMut<NextState<Menu>>) {
    next.set(Menu::LevelUp);
}

fn spawn_level_up_menu(mut commands: Commands, assets: Res<MenuAssets>, xp: Single<&Xp>) {
    commands.spawn((
        ui_root("LevelUp Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::LevelUp),
        children![
            header(&assets, format!("Level up {}", xp.level)),
            button(&assets, "Damage ++", damage_up),
            button(&assets, "Bullet speed ++", inc_attack_speed),
            button(&assets, "Fire rate ++", inc_fire_rate),
            button(&assets, "Health max ++", inc_health_max),
            button(&assets, "Skip", skip_menu),
        ],
    ));
}

fn damage_up(
    _: On<Pointer<Click>>,
    mut next: ResMut<NextState<Menu>>,
    mut xp: Single<&mut Xp>,
    mut slots: Single<&mut WeaponSlots>,
) {
    xp.level_up();
    let Some(w) = slots.slots.get_mut(0) else {
        return;
    };
    w.inc_damage(0.10);
    next.set(Menu::None);
}

fn inc_attack_speed(
    _: On<Pointer<Click>>,
    mut next: ResMut<NextState<Menu>>,
    mut xp: Single<&mut Xp>,
    mut slots: Single<&mut WeaponSlots>,
) {
    xp.level_up();
    let Some(w) = slots.slots.get_mut(0) else {
        return;
    };
    w.inc_speed(0.10);
    next.set(Menu::None);
}

fn inc_fire_rate(
    _: On<Pointer<Click>>,
    mut next: ResMut<NextState<Menu>>,
    mut xp: Single<&mut Xp>,
    mut slots: Single<&mut WeaponSlots>,
) {
    xp.level_up();
    let Some(w) = slots.slots.get_mut(0) else {
        return;
    };
    w.inc_fire_rate(0.50);
    next.set(Menu::None);
}

fn inc_health_max(
    _: On<Pointer<Click>>,
    mut next: ResMut<NextState<Menu>>,
    mut xp: Single<&mut Xp>,
    mut health: Single<&mut Health, With<Player>>,
) {
    xp.level_up();
    health.inc_max_health(0.10);
    next.set(Menu::None);
}

fn skip_menu(_: On<Pointer<Click>>, mut next: ResMut<NextState<Menu>>, mut xp: Single<&mut Xp>) {
    xp.skip();
    next.set(Menu::None);
}
