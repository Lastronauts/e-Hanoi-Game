use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::DiskNumber;

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
pub struct SpriteEntity;

#[derive(Component)]
pub struct TextEntity;

#[derive(Component)]
pub struct Disk(i32);

pub enum DiskCondition {
    Placed(i32),
    Lifted,
}

#[derive(Component)]
pub struct Position {
    pub rod: i32,
    pub height: DiskCondition,
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
    for i in disk_order {
        match die.sample(&mut rng) {
            0 => {
                // 左のポールのディスク
                commands
                    .spawn_bundle(disk_sprite(disk_color[i as usize], i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: 0,
                        height: DiskCondition::Placed(rods.left.disk_num),
                    })
                    .insert(GameEntity)
                    .insert(SpriteEntity);

                // 左のポールのディスクの文字
                commands
                    .spawn_bundle(disk_text(text_style.clone(), text_alignment, i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: 0,
                        height: DiskCondition::Placed(rods.left.disk_num),
                    })
                    .insert(GameEntity)
                    .insert(TextEntity);

                rods.left.disks.push(i);
                rods.left.disk_num += 1;
            }
            1 => {
                // 真ん中のポールのディスク
                commands
                    .spawn_bundle(disk_sprite(disk_color[i as usize], i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: 1,
                        height: DiskCondition::Placed(rods.center.disk_num),
                    })
                    .insert(GameEntity)
                    .insert(SpriteEntity);

                // 真ん中のポールのディスクの文字
                commands
                    .spawn_bundle(disk_text(text_style.clone(), text_alignment, i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: 1,
                        height: DiskCondition::Placed(rods.center.disk_num),
                    })
                    .insert(GameEntity)
                    .insert(TextEntity);

                rods.center.disk_num += 1;
                rods.center.disks.push(i);
            }
            2 => {
                commands
                    // 右のポールのディスク
                    .spawn_bundle(disk_sprite(disk_color[i as usize], i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: 2,
                        height: DiskCondition::Placed(rods.right.disk_num),
                    })
                    .insert(GameEntity)
                    .insert(SpriteEntity);

                // 右のポールのディスクの文字
                commands
                    .spawn_bundle(disk_text(text_style.clone(), text_alignment, i))
                    .insert(Disk(i))
                    .insert(Position {
                        rod: 2,
                        height: DiskCondition::Placed(rods.right.disk_num),
                    })
                    .insert(GameEntity)
                    .insert(TextEntity);

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

pub fn disk_movement(
    mut text: Query<(&Position, &mut Transform), With<TextEntity>>,
    mut rect: Query<(&Position, &mut Transform), Without<TextEntity>>,
) {
    for (pos, mut transform) in text.iter_mut() {
        match pos.height {
            DiskCondition::Placed(i) => {
                transform.translation =
                    Vec3::new((-400 + pos.rod * 400) as f32, (-240 + i * 50) as f32, 10.0)
            }
            DiskCondition::Lifted => {
                transform.translation = Vec3::new((-400 + pos.rod * 400) as f32, 240.0, 10.0)
            }
        }
    }
    for (pos, mut transform) in rect.iter_mut() {
        match pos.height {
            DiskCondition::Placed(i) => {
                transform.translation =
                    Vec3::new((-400 + pos.rod * 400) as f32, (-240 + i * 50) as f32, 9.0)
            }
            DiskCondition::Lifted => {
                transform.translation = Vec3::new((-400 + pos.rod * 400) as f32, 240.0, 9.0)
            }
        }
    }
}
