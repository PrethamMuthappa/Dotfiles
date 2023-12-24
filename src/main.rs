use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_pos(query: Query<(Entity, &mut Position)>) {
    for (entity, position) in query.iter() {
        println!("entity {:?} is at position {:?}", entity, position);
    }
}

pub struct SpacePlugin;
impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
        app.add_systems(Update, (update_position, print_pos));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpacePlugin)
        .run();
}
