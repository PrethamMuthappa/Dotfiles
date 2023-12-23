use bevy::prelude::*;

fn helloword() {
    println!("hello bevy");
}

//adding components

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("pretham".to_lowercase())));
    commands.spawn((Person, Name("pretham".to_lowercase())));
    commands.spawn((Person, Name("pretham".to_lowercase())));
}

fn greet(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (helloword, greet));
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
