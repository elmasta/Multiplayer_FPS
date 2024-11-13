use bevy::{
    color::palettes::css::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::PlayersList;

#[derive(Component)]
pub struct TextChanges;

#[derive(Component)]
pub struct Scores;

pub fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/Ubuntu-B.ttf");
    let root_uinode = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .id();

   

    let right_column = commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::End,
            flex_grow: 1.,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..default()
        },
        ..default()
    }).with_children(|builder| {

        builder.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "Score",
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.0,
                        color: Color::Srgba(RED),
                        ..default()
                    },
                )]),
            Scores,
        ));

        builder.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "This text changes in the bottom right",
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.0,
                        color: Color::Srgba(RED),
                        ..default()
                    },
                )]),
            TextChanges,
        ));
    })
    .id();

    commands
        .entity(root_uinode)
        .push_children(&[right_column]);
}

pub fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    players_list: Res<PlayersList>,
    mut fps_query: Query<&mut Text, With<TextChanges>>,
    mut score_query: Query<&mut Text,( With<Scores>, Without<TextChanges>)>,
) {
    for mut text in &mut fps_query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) =
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[0].value = format!(
            "{fps:.1} fps, {frame_time:.3} ms/frame",
        );
    }

     for mut text in &mut score_query {
        let mut score_str = String::from("SCORES\n");
        for player in players_list.players.iter(){
            let name = &player.name;
            let pts = player.score;
            score_str += format!(
            "{name} : {pts} pts \n").as_str()
        }

        text.sections[0].value = score_str;
     }
}
