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

pub fn print_positions(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Position, With<Boid>>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for position in query.iter() {
        println!("Position: {:?}", position);
    }
}

pub fn set_pos_vel_to_transform(mut query: Query<(&Position, &mut Transform), With<Boid>>) {
    for (position, mut transform) in query.iter_mut() {
        position.set_transform(&mut transform);
    }
}

pub fn apply_avoidance(behavior: Res<BoidBehavior>, mut query: Query<(&Position, &mut Velocity), With<Boid>>){
    let mut combinations = query.iter_combinations_mut();
    while let Some([(boid_a_pos, mut boid_a_vel), (boid_b_pos, mut boid_b_vel)]) = combinations.fetch_next() {
        let distance = boid_a_pos.vec.distance(boid_b_pos.vec);
        if distance < behavior.avoidance_radius {
            continue;
        }
        let force_mag = behavior.avoidance_force / distance;
        let force = (boid_a_pos.vec - boid_b_pos.vec) * force_mag;
        boid_a_vel.vec += force;
        boid_b_vel.vec -= force;
    }
}