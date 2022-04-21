mod setup;
mod state;
use bevy::prelude::*;
use state::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "e-Hanoi".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(home::ButtonNow(home::ButtonMarker::Free))
        .insert_resource(DiskNumber(8))
        .insert_resource(game::Rods {
            left: game::Rod {
                disk_num: 0,
                disks: Vec::new(),
            },
            center: game::Rod {
                disk_num: 0,
                disks: Vec::new(),
            },
            right: game::Rod {
                disk_num: 0,
                disks: Vec::new(),
            },
        })
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Home)
        .add_startup_system(setup::camera)
        .add_system_set(SystemSet::on_enter(AppState::Home).with_system(home::spawn_entities))
        .add_system_set(
            SystemSet::on_update(AppState::Home)
                .with_system(home::cursol_input.label(home::Choosing::CursorInput))
                .with_system(
                    home::cursor_movement
                        .label(home::Choosing::CursorMovement)
                        .after(home::Choosing::CursorInput),
                )
                .with_system(
                    home::choosing_input
                        .label(home::Choosing::ChoosingInput)
                        .after(home::Choosing::CursorMovement),
                ),
        )
        .add_system_set(SystemSet::on_exit(AppState::Home).with_system(home::despawn_entities))
        .add_system_set(SystemSet::on_enter(AppState::Free).with_system(game::spawn_entities))
        .add_system_set(SystemSet::on_update(AppState::Free).with_system(game::disk_movement))
        .run();
}
