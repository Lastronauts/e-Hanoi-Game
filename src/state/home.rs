use super::{game, AppState, SpaceNum};
use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
pub struct HomeEntity;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum ButtonMarker {
    Exit,
    Free,
    Ranking,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Choosing {
    CursorInput,
    CursorMovement,
    ChoosingInput,
}

pub struct ButtonNow(pub ButtonMarker);

pub fn spawn_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button = vec![
        ("Exit", -120.0, Color::YELLOW_GREEN, ButtonMarker::Exit),
        ("Free", -180.0, Color::ORANGE, ButtonMarker::Free),
        ("Ranking", -240.0, Color::RED, ButtonMarker::Ranking),
    ];
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 300.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    for (tex, tra, col, mark) in button.iter() {
        // タイトル画面にある塔のディスクの文字
        commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    *tex,
                    TextStyle {
                        font_size: 50.0,
                        ..text_style.clone()
                    },
                    text_alignment,
                ),
                transform: Transform {
                    translation: Vec3::new(0.0, *tra, 10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(HomeEntity)
            .insert(*mark);

        // タイトル画面にある塔のディスク
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: *col,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, *tra, 9.0),
                    scale: Vec3::new(*tra, 60.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(HomeEntity)
            .insert(*mark);
    }

    // タイトル画面にある塔のポール
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BISQUE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -150.0, 8.0),
                scale: Vec3::new(40.0, 230.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HomeEntity);

    // タイトル画面にある"e-Hanoi"のテキスト
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("e-Hanoi", text_style, text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 150.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HomeEntity);
}

pub fn cursor_input(mut button_now: ResMut<ButtonNow>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Down) {
        match button_now.0 {
            ButtonMarker::Exit => button_now.0 = ButtonMarker::Free,
            ButtonMarker::Free => button_now.0 = ButtonMarker::Ranking,
            ButtonMarker::Ranking => {}
        }
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        match button_now.0 {
            ButtonMarker::Exit => {}
            ButtonMarker::Free => button_now.0 = ButtonMarker::Exit,
            ButtonMarker::Ranking => button_now.0 = ButtonMarker::Free,
        }
    }
}

pub fn cursor_movement(mut texts: Query<(&mut Text, &ButtonMarker)>, button_now: Res<ButtonNow>) {
    for (mut text, mark) in texts.iter_mut() {
        if *mark == button_now.0 {
            (*text).sections[0].style.color = Color::INDIGO;
            (*text).sections[0].style.font_size = 60.0;
        } else {
            (*text).sections[0].style.color = Color::WHITE;
            (*text).sections[0].style.font_size = 50.0;
        }
    }
}

pub fn choosing_input(
    button_now: Res<ButtonNow>,
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<bevy::app::AppExit>,
    mut app_state: ResMut<State<AppState>>,
    mut is_ranking: ResMut<game::IsRanking>,
    mut space_num: ResMut<SpaceNum>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if space_num.0 < 1 {
            space_num.0 += 1;
        } else {
            match button_now.0 {
                ButtonMarker::Exit => {
                    event_writer.send(AppExit);
                }
                ButtonMarker::Free => {
                    is_ranking.0 = false;
                    app_state.set(AppState::CountDown).unwrap();
                }
                ButtonMarker::Ranking => {
                    is_ranking.0 = true;
                    app_state.set(AppState::CountDown).unwrap();
                }
            }
        }
    }
}

pub fn despawn_entities(mut commands: Commands, home_entities: Query<Entity, With<HomeEntity>>) {
    for ent in home_entities.iter() {
        commands.entity(ent).despawn();
    }
}
