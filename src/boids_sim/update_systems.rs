use super::components::*;
use super::prelude::*;

pub fn add_velocity_to_position(
    //time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    let time = Time::default();
    for (mut position, velocity) in query.iter_mut() {
        position.add_velocity(&velocity, &time);
    }
}

pub fn print_positions(query: Query<&Position>) {
    for position in query.iter() {
        println!("Position: {:?}", position);
    }
}