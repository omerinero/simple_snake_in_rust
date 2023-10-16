use bevy::prelude::*;
use bevy::prelude::KeyCode::Back;
use bevy::ui::widget::UiImageSize;
use crate::app_states::AppState;
use crate::commons::*;


#[derive(Resource)]
pub struct MenuData {
    main_menu_entity: Entity,
}

pub fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::GameScreen);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut main_menu = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        }).with_children(|parent| {


        parent.spawn(ImageBundle {
            image: UiImage::new(asset_server.load("texture/snake.png")),
            style: Style {
              width: Val::Percent(60.),
              height: Val::Percent(60.),
                ..default()
            },
            ..default()
        });

        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
    }).id();

    commands.insert_resource(MenuData { main_menu_entity: main_menu });
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.main_menu_entity).despawn_recursive();
}