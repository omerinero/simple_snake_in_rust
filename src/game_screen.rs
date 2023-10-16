use bevy::a11y::accesskit::Role::Image;
use bevy::a11y::AccessKitEntityExt;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy::render::render_resource::{ShaderType, Texture};
use bevy::ui::widget::UiImageSize;
use bevy::window::PrimaryWindow;
use rand::random;
use crate::app_states::AppState;
use crate::commons::*;


const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);


#[derive(Event)]
pub struct GrowthEvent;

#[derive(Component, Resource, Clone, Copy, PartialEq, Eq)]
pub struct Food;

#[derive(Default, Resource)]
pub struct LastTailPosition(Option<Position>);

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

pub fn position_translation(mut window: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.single().width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.single().height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

pub fn size_scaling(mut window: Query<&mut Window>, mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.single().width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.single().height(),
            1.0,
        );
    }
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (food, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(food).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_movement(
    mut next_state: ResMut<NextState<AppState>>,
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            next_state.set(AppState::GameOverScreen);
        }

        if segment_positions.contains(&head_pos) {
            next_state.set(AppState::GameOverScreen);
        }

        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

pub fn food_spawner(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        },
                Food
        ))

        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

pub fn clean_food(mut commands: Commands, query: Query<Entity, With<Food>>) {
    for food_entity in query.iter() {
        commands.entity(food_entity).despawn();
    }
}

pub fn clean_tail(mut commands: Commands, query: Query<Entity, With<SnakeSegment>>) {
    for segment_entity in query.iter() {
        commands.entity(segment_entity).despawn();
    }
}

pub fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}


pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}