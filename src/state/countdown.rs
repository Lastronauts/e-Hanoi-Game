use bevy::prelude::*;

#[derive(Component)]
pub struct time(pub i32);

pub fn spawn(commands: Commands, asset_server: Res<AssetServer>) {
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
    commands.
        spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "3",
                text_style,
                text_alignment,
            ),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(time(3))
}

pub fn count_down(mut ent: Query<(&mut Text, &time)>) {
    for (mut text, time) in ent.iter_mut() {
        (*text).sections[0]
    }
}
