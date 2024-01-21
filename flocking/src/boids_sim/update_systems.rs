use std::f32::consts::PI;
use super::components::*;
use super::prelude::*;

pub fn apply_steering_to_velocity(
    behavior: Res<BoidBehavior>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut SteeringForce)>) {
    for (mut velocity, mut steering_force) in query.iter_mut() {
        let clamped_force = steering_force.angular_acceleration.clamp(
            -behavior.max_angular_acceleration,
            behavior.max_angular_acceleration,
        );
        // rad/s^2 * s = rad/s
        let impulse = clamped_force * time.delta_seconds();
        todo!("figure out what steering force actually means");
        steering_force.angular_acceleration = 0.0;
    }
}

pub fn add_velocity_to_position(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    for (mut position, velocity) in query.iter_mut() {
        position.add_velocity(&velocity, &time);
    }
}

pub fn apply_drag(
    behavior: Res<BoidBehavior>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity)>,
) {
    for mut velocity in query.iter_mut() {
        let dist = velocity.vec.length();
        let force_scalar = dist * dist * behavior.combined_drag_coefficient;
        let force_vector = velocity.vec.normalize() * force_scalar;
        velocity.vec -= force_vector * time.delta_seconds();
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

pub fn set_pos_vel_to_transform(mut query: Query<(&Position, &Velocity, &mut Transform), With<Boid>>) {
    for (position, velocity, mut transform) in query.iter_mut() {
        position.set_transform(&mut transform);
        transform.rotation = Quat::from_rotation_z(velocity.vec.y.atan2(velocity.vec.x));
    }
}

pub fn apply_avoidance(behavior: Res<BoidBehavior>, mut query: Query<(&Position, &mut Velocity), With<Boid>>){
    let mut combinations = query.iter_combinations_mut();
    while let Some([(boid_a_pos, mut boid_a_vel), (boid_b_pos, mut boid_b_vel)]) = combinations.fetch_next() {
        let distance = boid_a_pos.vec.distance(boid_b_pos.vec);
        if distance > behavior.avoidance_radius || distance <= EPSILON {
            continue;
        }
        let mut force_mag = behavior.avoidance_force / distance;
        force_mag = force_mag.min(behavior.max_avoidance_force);
        let force = (boid_a_pos.vec - boid_b_pos.vec) * force_mag;
        boid_a_vel.vec += force;
        boid_b_vel.vec -= force;
    }
}

pub fn apply_flock_info(behavior: Res<BoidBehavior>, mut query: Query<(&Position,  &Velocity, &mut BoidFlockInfo), With<Boid>>){
    for (pos, vel, mut flock_info) in query.iter_mut() {
        flock_info.reset(pos, vel);
    }
    let mut combinations = query.iter_combinations_mut();
    while let Some([
                   (boid_a_pos, boid_a_vel, mut boid_a_flock_info),
                   (boid_b_pos, boid_b_vel, mut boid_b_flock_info)]) = combinations.fetch_next() {
        let distance = boid_a_pos.vec.distance(boid_b_pos.vec);
        if distance > behavior.flocking_radius {
            continue;
        }
        boid_a_flock_info.append_boid(boid_b_pos, boid_b_vel);
        boid_b_flock_info.append_boid(boid_a_pos, boid_a_vel);
    }
}

pub fn apply_cohesion(behavior: Res<BoidBehavior>, mut query: Query<(&Position, &mut Velocity, &BoidFlockInfo), With<Boid>>){
    for (position, mut velocity, flock_info) in query.iter_mut() {
        let center = flock_info.average_position();
        let delta = center - position.vec;
        let delta_len = delta.length();
        if delta_len <= EPSILON {
            continue;
        }
        let mut force_mag = behavior.cohesion_force * delta.length();
        force_mag = force_mag.min(behavior.max_cohesion_force);
        let force = delta.normalize() * force_mag;
        velocity.vec += force;
    }
}

pub fn apply_alignment(behavior: Res<BoidBehavior>, time: Res<Time>, mut query: Query<(&Position, &mut Velocity, &BoidFlockInfo), With<Boid>>){
    for (position, mut velocity, flock_info) in query.iter_mut() {
        let average_velocity = flock_info.average_velocity();
        let average_direction = average_velocity.angle_between(Vec2::X);
        let self_direction = velocity.vec.angle_between(Vec2::X);
        let direction_delta = average_direction - self_direction;
        if direction_delta <= EPSILON {
            continue;
        }
        // rad
        let mut force_mag = behavior.alignment_force * direction_delta * time.delta_seconds();
        force_mag = force_mag.min(behavior.max_alignment_force);
        velocity.vec = Vec2::from_angle(force_mag).rotate(velocity.vec);
    }
}

pub fn apply_wander(behavior: Res<BoidBehavior>, time: Res<Time>, mut query: Query<(&BoidSeed, &mut Velocity), With<Boid>>){
    for (seed, mut velocity) in query.iter_mut() {
        let t = (time.elapsed_seconds() * behavior.wander_frequency + seed.0) * PI * 2.0;
        let current_dir = velocity.vec.y.atan2(velocity.vec.x);
        let new_dir = current_dir + t.sin() * behavior.wander_angle_range;
        let force = Vec2::new(new_dir.cos(), new_dir.sin()) * behavior.wander_force;
        velocity.vec += force;
    }
}