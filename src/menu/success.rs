use crate::{
    cleanup::{cleanup, CleanUp},
    AppState,
};
use bevy::prelude::*;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::SuccessMenu)))
            .add_system(click.in_set(OnUpdate(AppState::SuccessMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::SuccessMenu)));
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::from_section(
                    "Success",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        margin: UiRect {
                            top: Val::Percent(4.0),
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Next",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..Default::default()
                    });
                });
        })
        .insert(CleanUp);
}

pub fn click(mut state: ResMut<NextState<AppState>>, input: Query<&Interaction, With<Button>>) {
    for interaction in input.iter() {
        if *interaction == Interaction::Clicked {
            state.set(AppState::LevelMenu);
        }
    }
}