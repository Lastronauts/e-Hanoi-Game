use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::{AppState, DiskNumber};

pub struct IsRanking(pub bool);

pub struct Timer(pub Time);

pub struct StartTime(pub f64);

pub struct TimeNow(pub i64);

pub struct IsHolding(pub bool);

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct IsCursored(pub bool);

#[derive(Component)]
pub struct IsTop(pub bool);

pub struct Rod {
    pub disk_num: i32,
    pub disks: Vec<i32>,
}

pub struct Rods {
    pub left: Rod,
    pub center: Rod,
    pub right: Rod,
}

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Disk(i32);

#[derive(PartialEq, Clone)]
pub enum DiskCondition {
    Placed(i32),
    Lifted,
}

pub struct WhereDiskWas(pub Position);

#[derive(PartialEq, Clone, Component)]
pub enum WhichRod {
    Left,
    Center,
    Right,
}

pub struct CursorRod(pub WhichRod);

#[derive(Component, Clone)]
pub struct Position {
    pub rod: WhichRod,
    pub height: DiskCondition,
}

#[derive(Component)]
pub struct RectPosition(Position);

#[derive(Component)]
pub struct TextPosition(Position);

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Label {
    TopDisk,
    CursoredDisk,
    CursoredDiskChange,
    CursorChange,
    Input,
    IsClear,
}

pub fn spawn_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    disk_number: Res<DiskNumber>,
    mut rods: ResMut<Rods>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 50.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let disk_color = vec![
        Color::BLUE,
        Color::YELLOW_GREEN,
        Color::PINK,
        Color::PURPLE,
        Color::ORANGE,
        Color::MAROON,
        Color::LIME_GREEN,
        Color::RED,
        Color::SILVER,
        Color::VIOLET,
    ];
    let mut rng = thread_rng();
    let mut disk_order: Vec<i32> = (0..disk_number.0).collect();
    let die = Uniform::<i32>::from(0..3);
    disk_order.shuffle(&mut rng);
    for i in [0, 1, 2].iter() {
        // 三つのロッド
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BISQUE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new((-400 + i * 400) as f32, -25.0, 8.0),
                    scale: Vec3::new(40.0, 490.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GameEntity);
    }

    // timeを表示するテキスト
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    color: Color::BLACK,
                    font_size: 40.0,
                    ..text_style.clone()
                },
                TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Right,
                },
            ),
            transform: Transform {
                translation: Vec3::new(550.0, 330.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(TimeText);

    // どのポールを選んでいるかを示すスプライト
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GREEN,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(50.0, 500.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(Cursor);

    for i in disk_order {
        match die.sample(&mut rng) {
            0 => {
                // 左のポールのディスク
                commands
                    .spawn_bundle(disk_sprite(disk_color[i as usize], i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: WhichRod::Left,
                        height: DiskCondition::Placed(rods.left.disk_num),
                    })
                    .insert(IsCursored(false))
                    .insert(IsTop(false))
                    .insert(GameEntity);

                // 左のポールのディスクの文字
                commands
                    .spawn_bundle(disk_text(text_style.clone(), text_alignment, i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: WhichRod::Left,
                        height: DiskCondition::Placed(rods.left.disk_num),
                    })
                    .insert(IsCursored(false))
                    .insert(IsTop(false))
                    .insert(GameEntity);

                rods.left.disks.push(i);
                rods.left.disk_num += 1;
            }
            1 => {
                // 真ん中のポールのディスク
                commands
                    .spawn_bundle(disk_sprite(disk_color[i as usize], i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: WhichRod::Center,
                        height: DiskCondition::Placed(rods.center.disk_num),
                    })
                    .insert(IsCursored(false))
                    .insert(IsTop(false))
                    .insert(GameEntity);

                // 真ん中のポールのディスクの文字
                commands
                    .spawn_bundle(disk_text(text_style.clone(), text_alignment, i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: WhichRod::Center,
                        height: DiskCondition::Placed(rods.center.disk_num),
                    })
                    .insert(IsCursored(false))
                    .insert(IsTop(false))
                    .insert(GameEntity);

                rods.center.disk_num += 1;
                rods.center.disks.push(i);
            }
            2 => {
                // 右のポールのディスク
                commands
                    .spawn_bundle(disk_sprite(disk_color[i as usize], i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: WhichRod::Right,
                        height: DiskCondition::Placed(rods.right.disk_num),
                    })
                    .insert(IsCursored(false))
                    .insert(IsTop(false))
                    .insert(GameEntity);

                // 右のポールのディスクの文字
                commands
                    .spawn_bundle(disk_text(text_style.clone(), text_alignment, i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: WhichRod::Right,
                        height: DiskCondition::Placed(rods.right.disk_num),
                    })
                    .insert(IsCursored(false))
                    .insert(IsTop(false))
                    .insert(GameEntity);

                rods.right.disk_num += 1;
                rods.right.disks.push(i);
            }
            i => eprintln!("Unexpected value detected: {}", i),
        }
    }
}

fn disk_sprite(disk_color: Color, disk: i32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: disk_color,
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(80.0 + (disk as f32) * 30.0, 50.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn disk_text(text_style: TextStyle, text_alignment: TextAlignment, disk: i32) -> Text2dBundle {
    Text2dBundle {
        text: Text::with_section(
            format!("{}", disk + 1),
            TextStyle {
                font_size: 50.0,
                ..text_style.clone()
            },
            text_alignment,
        ),
        ..Default::default()
    }
}

pub fn setup(
    mut cursor_rod: ResMut<CursorRod>,
    mut timer: ResMut<Timer>,
    mut start_time: ResMut<StartTime>,
    mut rods: ResMut<Rods>,
) {
    rods.left.disks = Vec::new();
    rods.left.disk_num = 0;
    rods.center.disks = Vec::new();
    rods.center.disk_num = 0;
    rods.right.disks = Vec::new();
    rods.right.disk_num = 0;
    cursor_rod.0 = WhichRod::Center;
    timer.0.update();
    start_time.0 = timer.0.time_since_startup().as_secs_f64();
}

pub fn cursored_disk(
    cursor_rod: Res<CursorRod>,
    is_holding: Res<IsHolding>,
    mut disks: Query<(&Position, &mut IsCursored, &IsTop)>,
) {
    if is_holding.0 {
        for (pos, mut cur_boo, _) in disks.iter_mut() {
            if pos.height == DiskCondition::Lifted {
                cur_boo.0 = true;
            } else {
                cur_boo.0 = false;
            }
        }
    } else {
        for (pos, mut cur_boo, top_boo) in disks.iter_mut() {
            if pos.rod == cursor_rod.0 && top_boo.0 {
                cur_boo.0 = true;
            } else {
                cur_boo.0 = false;
            }
        }
    }
}

pub fn top_disk(mut disks: Query<(&Position, &mut IsTop)>) {
    let mut left_top = -1;
    let mut center_top = -1;
    let mut right_top = -1;
    for (pos, _) in disks.iter() {
        if let DiskCondition::Placed(i) = pos.height {
            match pos.rod {
                WhichRod::Left => {
                    if i > left_top {
                        left_top = i;
                    }
                }
                WhichRod::Center => {
                    if i > center_top {
                        center_top = i;
                    }
                }
                WhichRod::Right => {
                    if i > right_top {
                        right_top = i;
                    }
                }
            }
        }
    }
    for (pos, mut top_boo) in disks.iter_mut() {
        if let DiskCondition::Placed(i) = pos.height {
            match pos.rod {
                WhichRod::Left => {
                    if i == left_top {
                        top_boo.0 = true;
                    } else {
                        top_boo.0 = false;
                    }
                }
                WhichRod::Center => {
                    if i == center_top {
                        top_boo.0 = true;
                    } else {
                        top_boo.0 = false;
                    }
                }
                WhichRod::Right => {
                    if i == right_top {
                        top_boo.0 = true;
                    } else {
                        top_boo.0 = false;
                    }
                }
            }
        } else {
            top_boo.0 = false;
        }
    }
}

pub fn output_time(
    mut time: ResMut<Timer>,
    start_time: Res<StartTime>,
    mut time_now: ResMut<TimeNow>,
    mut text: Query<&mut Text, With<TimeText>>,
) {
    time.0.update();
    time_now.0 = (time.0.time_since_startup().as_secs_f64() - start_time.0) as i64;
    for mut tex in text.iter_mut() {
        tex.sections[0].value = format!("{}", time_now.0,);
    }
}

pub fn cursored_disk_change(mut disks: Query<(&IsCursored, &mut Text)>) {
    for (cur_boo, mut text) in disks.iter_mut() {
        if cur_boo.0 {
            (*text).sections[0].style.color = Color::BLACK;
            (*text).sections[0].style.font_size = 70.0;
        } else {
            (*text).sections[0].style.color = Color::WHITE;
            (*text).sections[0].style.font_size = 50.0;
        }
    }
}

pub fn cursor_change(cursor_now: Res<CursorRod>, mut cursor: Query<&mut Transform, With<Cursor>>) {
    match cursor_now.0 {
        WhichRod::Left => {
            for mut tra in cursor.iter_mut() {
                (*tra).translation = Vec3::new(-400.0, -25.0, 7.0)
            }
        }
        WhichRod::Center => {
            for mut tra in cursor.iter_mut() {
                (*tra).translation = Vec3::new(0.0, -25.0, 7.0)
            }
        }
        WhichRod::Right => {
            for mut tra in cursor.iter_mut() {
                (*tra).translation = Vec3::new(400.0, -25.0, 7.0)
            }
        }
    }
}

pub fn text_translation(mut text: Query<(&Position, &mut Transform), With<Text>>) {
    for (pos, mut transform) in text.iter_mut() {
        let rod_num = match pos.rod {
            WhichRod::Left => 0,
            WhichRod::Center => 1,
            WhichRod::Right => 2,
        };
        match pos.height {
            DiskCondition::Placed(i) => {
                transform.translation =
                    Vec3::new((-400 + rod_num * 400) as f32, (-270 + i * 50) as f32, 10.0)
            }
            DiskCondition::Lifted => {
                transform.translation = Vec3::new((-400 + rod_num * 400) as f32, 260.0, 10.0)
            }
        }
    }
}

pub fn rect_translation(mut rect: Query<(&Position, &mut Transform), With<Sprite>>) {
    for (pos, mut transform) in rect.iter_mut() {
        let rod_num = match pos.rod {
            WhichRod::Left => 0,
            WhichRod::Center => 1,
            WhichRod::Right => 2,
        };
        match pos.height {
            DiskCondition::Placed(i) => {
                transform.translation =
                    Vec3::new((-400 + rod_num * 400) as f32, (-270 + i * 50) as f32, 9.0)
            }
            DiskCondition::Lifted => {
                transform.translation = Vec3::new((-400 + rod_num * 400) as f32, 260.0, 9.0)
            }
        }
    }
}

pub fn input(
    mut cursor_rod: ResMut<CursorRod>,
    keyboard_input: Res<Input<KeyCode>>,
    mut is_holding: ResMut<IsHolding>,
    mut where_disk_was: ResMut<WhereDiskWas>,
    mut disks: Query<(&mut Position, &IsCursored, &Disk, &IsTop)>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        match cursor_rod.0 {
            WhichRod::Left => cursor_rod.0 = WhichRod::Right,
            WhichRod::Center => cursor_rod.0 = WhichRod::Left,
            WhichRod::Right => cursor_rod.0 = WhichRod::Center,
        }
        if is_holding.0 {
            for (mut pos, _, _, _) in disks.iter_mut() {
                if pos.height == DiskCondition::Lifted {
                    pos.rod = cursor_rod.0.clone();
                }
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        match cursor_rod.0 {
            WhichRod::Left => cursor_rod.0 = WhichRod::Center,
            WhichRod::Center => cursor_rod.0 = WhichRod::Right,
            WhichRod::Right => cursor_rod.0 = WhichRod::Left,
        }
        if is_holding.0 {
            for (mut pos, _, _, _) in disks.iter_mut() {
                if pos.height == DiskCondition::Lifted {
                    pos.rod = cursor_rod.0.clone();
                }
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        if is_holding.0 {
            is_holding.0 = false;
            let mut top_num = 11;
            let mut disk_num = -1;
            for (pos, _, num, top_boo) in disks.iter() {
                if pos.rod == cursor_rod.0 && top_boo.0 {
                    top_num = num.0;
                    if let DiskCondition::Placed(i) = pos.height {
                        disk_num = i;
                    }
                }
            }
            for (mut pos, _, num, _) in disks.iter_mut() {
                if pos.height == DiskCondition::Lifted {
                    if num.0 > top_num {
                        cursor_rod.0 = where_disk_was.0.rod.clone();
                        *pos = where_disk_was.0.clone();
                    } else {
                        pos.height = DiskCondition::Placed(disk_num + 1);
                    }
                }
            }
        } else {
            is_holding.0 = true;
            for (mut pos, cur_boo, _, _) in disks.iter_mut() {
                if cur_boo.0 {
                    where_disk_was.0 = (*pos).clone();
                    pos.height = DiskCondition::Lifted;
                }
            }
        }
    }
}

pub fn is_clear(disks: Query<(&IsTop, &Disk), With<Text>>, mut app_state: ResMut<State<AppState>>) {
    let mut max = -1;
    for (top_boo, num) in disks.iter() {
        if top_boo.0 && max < num.0 {
            max = num.0;
        }
    }
    if max == 0 {
        app_state.set(AppState::Clear).unwrap();
    }
}

pub fn despawn_entities(mut commands: Commands, game_entities: Query<Entity, With<GameEntity>>) {
    for ent in game_entities.iter() {
        commands.entity(ent).despawn();
    }
}
