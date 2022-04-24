use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::graphql::send_clear_time;

use super::{
    game::{self, TimeNow},
    home::SpaceNum,
    AppState,
};

#[derive(Component)]
pub struct ClearEntity;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    clear_time: Res<TimeNow>,
    is_ranking: Res<game::IsRanking>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 300.0,
        color: Color::BLUE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    // クリアの文字
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("CLEAR!", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 130.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ClearEntity);

    // クリアタイム
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("clear time: {}s", clear_time.0),
                TextStyle {
                    font_size: 100.0,
                    ..text_style
                },
                text_alignment,
            ),
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ClearEntity);

    if is_ranking.0 {
        let time=clear_time.clone();
        let task = thread_pool.spawn(async move {
            let query_res = send_clear_time(time.0.clone());

            if let Err(err) = query_res {
                eprintln!("Error: failed to execute query");
                eprintln!("detail...\n{}", err);
            }
        });
        commands.spawn().insert(task);
    }
}

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut space_num: ResMut<SpaceNum>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        space_num.0 = 0;
        app_state.set(AppState::Home).unwrap();
    }
}

pub fn despawn(mut commands: Commands, clear_entities: Query<Entity, With<ClearEntity>>) {
    for ent in clear_entities.iter() {
        commands.entity(ent).despawn();
    }
}
