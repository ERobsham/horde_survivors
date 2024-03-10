use bevy::prelude::*;

use crate::GameState;
use super::{style::*, types::LoadingUpdate};

#[derive(Component, Debug, Default)]
struct LoadingMenu;

#[derive(Component, Debug, Default)]
struct LoadingBar;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_loading_ui)
            .add_systems(Update, update_loading_ui
                .run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), hide_loading_ui);
    }
}


fn setup_loading_ui(
    mut commands: Commands,
) {
    // ==== Main Loading Menu BG =====
    commands.spawn((
        NodeBundle {
            style: MAIN_WINDOW_BG_STYLE,
            background_color: MAIN_WINDOW_BG_COLOR,
            ..default()
        },
        LoadingMenu{},
    )).with_children(| parent | {
        parent.spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                width: Val::Percent(60.0),
                height: Val::Percent(85.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_content: AlignContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY.with_a(0.33)),
            ..default()
        }).with_children(|parent| {
            // ===== Title =====
            parent.spawn(menu_text("Horde Survivors", 68.0));

            // ===== Sub Title =====
            parent.spawn(menu_text("loading...", 24.0));

            // ===== Loading Bar =====
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(80.),
                    height: Val::Px(20.0),
                    
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,

                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Row,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                border_color:BorderColor(Color::WHITE),
                ..default()
            }).with_children(| parent | {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width:Val::Percent(0.0),
                            max_width:Val::Percent(100.0),
                            height:Val::Percent(100.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..default()
                    },
                    LoadingBar{},
                ));
            });
        });
    });
}


fn update_loading_ui(
    // mut commands: Commands,
    mut events: EventReader<LoadingUpdate>,
    mut q_bar: Query<&mut Style, With<LoadingBar>>,
) {
    for event in events.read() {
        let (loaded, total) = (event.0 as f32, event.1 as f32);
        let width = (loaded / total) * 100.;

        if let Ok(mut bar) = q_bar.get_single_mut() {
            bar.min_width = Val::Percent(width);
        }
    }
}

fn hide_loading_ui(
    mut commands: Commands,
    q_menu: Query<Entity, With<LoadingMenu>>,
) {
    if let Ok(menu_id) = q_menu.get_single() {
        commands.entity(menu_id).despawn_recursive();
    }
}
