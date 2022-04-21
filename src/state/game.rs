use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::DiskNumber;

pub struct Rod {
    pub DiskNum: i32,
    pub Disks: Vec<i32>,
}

pub struct Rods {
    pub left: Rod,
    pub center: Rod,
    pub right: Rod,
}

pub fn spawn_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    disk_number: Res<DiskNumber>,
    mut rods: ResMut<Rods>,
) {
    let mut rng = thread_rng();
    let mut disk_order: Vec<i32> = (1..=disk_number.0).collect();
    let die = Uniform::from(0..3);
    disk_order.shuffle(&mut rng);
    for i in disk_order {
        match die.sample(&mut rng) {
            0 => {
                rods.left.DiskNum += 1;
                rods.left.Disks.push(i);
            }
            1 => {
                rods.center.DiskNum += 1;
                rods.center.Disks.push(i);
            }
            2 => {
                rods.right.DiskNum += 1;
                rods.right.Disks.push(i);
            }
            i => eprintln!("Unexpected value detected: {}", i),
        }
    }
}
