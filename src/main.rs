// #![windows_subsystem = "windows"]

mod graphql;
mod setup;
mod state;
use bevy::{core::FixedTimestep, prelude::*};
use state::*;

fn main() {
    println!("main!");
    App::new()
        .insert_resource(WindowDescriptor {
            title: "e-Hanoi".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(home::ButtonNow(home::ButtonMarker::Free))
        .insert_resource(DiskNumber(8))
        .insert_resource(game::CursorRod(game::WhichRod::Center))
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
        .insert_resource(game::IsHolding(false))
        .insert_resource(game::WhereDiskWas(game::Position {
            rod: game::WhichRod::Center,
            height: game::DiskCondition::Placed(0),
        }))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Home)
        .add_startup_system(setup::camera)
        .add_system_set(SystemSet::on_enter(AppState::Home).with_system(home::spawn_entities))
        .add_system_set(
            SystemSet::on_update(AppState::Home)
                .with_system(home::cursor_input.label(home::Choosing::CursorInput))
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
        .add_system_set(SystemSet::on_enter(AppState::CountDown).with_system(countdown::spawn))
        .add_system_set(
            SystemSet::on_update(AppState::CountDown)
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(countdown::count_down),
        )
        .add_system_set(SystemSet::on_exit(AppState::CountDown).with_system(countdown::despawn))
        .add_system_set(
            SystemSet::on_enter(AppState::Free)
                .with_system(game::spawn_entities)
                .with_system(game::cursor_set),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Free)
                .with_system(game::top_disk.label(game::Label::TopDisk))
                .with_system(
                    game::cursored_disk
                        .label(game::Label::CursoredDisk)
                        .after(game::Label::TopDisk),
                )
                .with_system(
                    game::cursored_disk_change
                        .label(game::Label::CursoredDiskChange)
                        .after(game::Label::CursoredDisk),
                )
                .with_system(
                    game::input
                        .label(game::Label::Input)
                        .after(game::Label::CursoredDiskChange),
                )
                .with_system(game::rect_translation)
                .with_system(game::text_translation),
        )
        .add_system_set(SystemSet::on_exit(AppState::Free).with_system(game::despawn_entities))
        .run();
}
