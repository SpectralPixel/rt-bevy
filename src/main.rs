use bevy::prelude::*;
use rt_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SetupGamePlugin))
        .run();
}
