use crate::gameplay::enemy::EnemyDeathEvent;
use bevy::app::App;
use bevy::prelude::On;

pub fn plugin(app: &mut App) {
    app.world_mut()
        .add_observer(|enemy_death: On<EnemyDeathEvent>| {
            println!("{}", enemy_death.pos);
        });
}
