use bevy::prelude::*;

//
// COMPONENTS
//

    struct Person;
    struct Name(String);

//
// SYSTEMS
//

    // Startup systems are just like normal systems, but they run exactly once, before all other systems, 
    // right when our app starts. Let's use Commands to spawn some entities into our World 

    fn add_people(commands: &mut Commands) {
        commands
            .spawn((Person, Name("Chris One".to_string())))
            .spawn((Person, Name("Chris Two".to_string())))
            .spawn((Person, Name("Chris Three".to_string())));
    }

    // The parameters we pass in to a "system function" define what data the system runs on. 
    // In this case, greet_people will run on all entities with the Person and Name component.

    struct GreetTimer(Timer);

    fn greet_people(
        query: Query<&Name, With<Person>>,
        time: Res<Time>,
        mut timer: ResMut<GreetTimer>
    ) {

        println!("{}", time.seconds_since_startup());

        if !timer.0.tick(time.delta_seconds()).just_finished() {
            return;
        }

        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }

//
// PLUGINS
//

    pub struct HelloPlugin;
    impl Plugin for HelloPlugin {
        fn build (&self, app: &mut AppBuilder) {
            app
                .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
                .add_startup_system(add_people.system())
                .add_system(greet_people.system());
        }
    }

//
// MAIN
//

    fn main() {
        App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(HelloPlugin)
            .run();
    }