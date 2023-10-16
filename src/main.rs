#![allow(unused)]

mod menu_screen;
mod app_states;
mod commons;
mod gameover_screen;
mod game_screen;

use std::slice::Windows;
use std::time::Duration;
use bevy::app::AppLabel;
use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::{PrimaryWindow, WindowDestroyed, WindowResized, WindowResolution};
use rand::prelude::random;
use crate::app_states::AppState;
use crate::menu_screen::*;
use crate::commons::*;
use crate::game_screen::*;
use crate::gameover_screen::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake!".into(),
                    resolution: (500., 500.).into(),
                    ..default()
                }),
                ..default()
            })
        )
        //STARTUP
        .add_systems(Startup, (
            setup_camera
        ))
        .add_systems(OnEnter(AppState::MenuScreen), setup_menu)
        .add_systems(OnExit(AppState::MenuScreen), cleanup_menu)
        .add_systems(Update,
                     menu.run_if(in_state(AppState::MenuScreen)
                     ))
        .add_systems(OnEnter(AppState::GameScreen), (
            spawn_snake
        ))
        .add_systems(Update,
                     (
                         snake_movement_input.before(snake_movement),
                         snake_movement.run_if(on_timer(Duration::from_secs_f64(0.150))),
                         food_spawner.run_if(on_timer(Duration::from_secs(3))),
                         snake_eating.after(snake_movement),
                         snake_growth.after(snake_eating),
                         position_translation,
                         size_scaling
                     ).run_if(in_state(AppState::GameScreen)))
        .add_systems(OnExit(AppState::GameScreen), (
            clean_food,
            clean_tail
            ))
        .add_systems(OnEnter(AppState::GameOverScreen), setup_game_over_menu)
        .add_systems(OnExit(AppState::GameOverScreen), cleanup_game_over_menu)
        .add_systems(Update, gameover_menu.run_if(in_state(AppState::GameOverScreen)))
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .run();
}


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}




