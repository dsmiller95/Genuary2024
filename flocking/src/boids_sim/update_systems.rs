use super::components::*;
use super::prelude::*;

pub fn add_velocity_to_position(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    for (mut position, velocity) in query.iter_mut() {
        position.add_velocity(&velocity, &time);
    }
}

pub fn print_positions(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Position>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for position in query.iter() {
        println!("Position: {:?}", position);
    }
}