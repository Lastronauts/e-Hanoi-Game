use bevy::prelude::*;

use super::AppState;

#[derive(Component)]
pub struct Time(pub i32);

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-ExtraBoldItalic.ttf");
    let text_style = TextStyle {
        font,
        font_size: 300.0,
        color: Color::RED,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("3", text_style, text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Time(3));
}

pub fn count_down(mut ent: Query<(&mut Text, &mut Time)>, mut app_state: ResMut<State<AppState>>) {
    for (mut text, mut tim) in ent.iter_mut() {
        tim.0 -= 1;
        if tim.0 < 1 {
            app_state.set(AppState::Game).unwrap();
        }
        (*text).sections[0].value = format!("{}", tim.0);
    }
}

pub fn despawn(mut commands: Commands, entities: Query<Entity, With<Time>>) {
    for ent in entities.iter() {
        commands.entity(ent).despawn();
    }
}
