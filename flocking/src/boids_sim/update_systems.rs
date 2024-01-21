use std::f32::consts::PI;
use std::ops::Mul;
use super::components::*;
use super::prelude::*;


pub fn add_velocity_to_position(
    time: Res<Time>,
    mut query: Query<(&mut Position, &mut Rotation, &Speed, &RotationalVelocity )>,
) {
    for (mut pos, mut rot, speed,  rot_vel) in query.iter_mut() {
        rot.radians = (rot.radians + rot_vel.radians_per_second * time.delta_seconds()) % (2.0 * PI);
        let velocity = speed.pixels_per_second * Vec2::from_angle(rot.radians);
        pos.vec += velocity * time.delta_seconds();
    }
}

pub fn apply_drag(
    behavior: Res<BoidBehavior>,
    time: Res<Time>,
    mut query: Query<(&mut RotationalVelocity)>,
) {
    for mut velocity in query.iter_mut() {
        let dist = velocity.radians_per_second;
        let force_scalar = dist * dist * behavior.combined_drag_coefficient;
        velocity.radians_per_second -= force_scalar * time.delta_seconds();
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

pub fn set_pos_vel_to_transform(mut query: Query<(&Position, &Rotation, &mut Transform), With<Boid>>) {
    for (pos, rot, mut transform) in query.iter_mut() {
        transform.translation.x = pos.vec.x;
        transform.translation.y = pos.vec.y;
        transform.rotation = Quat::from_rotation_z(rot.radians);
    }
}


fn steer_avoid_accel(current_direction: Vec2, avoid_direction: Vec2) -> f32 {
    steer_towards(current_direction, -avoid_direction)
}
/// returns a steering force between -PI and PI
fn steer_towards(current_direction: Vec2, attract_direction: Vec2) -> f32 {
    let a_to_b_angle = attract_direction.angle_between(current_direction);
    -a_to_b_angle
}

pub fn apply_avoidance(behavior: Res<BoidBehavior>, time: Res<Time>, mut query: Query<(&Position, &Rotation, &mut RotationalVelocity), With<Boid>>){
    let mut combinations = query.iter_combinations_mut();
    while let Some([
               (boid_a_pos, boid_a_rot, mut boid_a_vel),
               (boid_b_pos, boid_b_rot, mut boid_b_vel)
           ]) = combinations.fetch_next() {
        let distance = boid_a_pos.vec.distance(boid_b_pos.vec);
        if distance > behavior.avoidance_radius || distance <= EPSILON {
            continue;
        }
        let inverse_sq_dist = 1.0 / (distance * distance);
        let direction_a_to_b = (boid_b_pos.vec - boid_a_pos.vec).normalize();
        let direction_b_to_a = -direction_a_to_b;

        let a_steering = steer_avoid_accel(boid_a_rot.vec(), direction_a_to_b)
            .mul(behavior.avoidance_force * inverse_sq_dist)
            .min(behavior.max_avoidance_force);
        boid_a_vel.radians_per_second += a_steering * time.delta_seconds();

        let b_steering = steer_avoid_accel(boid_b_rot.vec(), direction_b_to_a)
            .mul(behavior.avoidance_force * inverse_sq_dist)
            .min(behavior.max_avoidance_force);
        boid_b_vel.radians_per_second += b_steering * time.delta_seconds();
    }
}

pub fn apply_flock_info(behavior: Res<BoidBehavior>, mut query: Query<(&Position, &Rotation, &mut BoidFlockInfo), With<Boid>>){
    for (pos, rot, mut flock_info) in query.iter_mut() {
        flock_info.reset(pos, rot);
    }
    let mut combinations = query.iter_combinations_mut();
    while let Some([
                   (boid_a_pos, boid_a_rot, mut boid_a_flock_info),
                   (boid_b_pos, boid_b_rot, mut boid_b_flock_info)]) = combinations.fetch_next() {
        let distance = boid_a_pos.vec.distance(boid_b_pos.vec);
        if distance > behavior.flocking_radius {
            continue;
        }
        boid_a_flock_info.append_boid(boid_b_pos, boid_b_rot);
        boid_b_flock_info.append_boid(boid_a_pos, boid_a_rot);
    }
}

pub fn apply_cohesion(behavior: Res<BoidBehavior>, mut query: Query<(&Position, &Rotation, &mut RotationalVelocity, &BoidFlockInfo), With<Boid>>){
    for (position, rot, mut velocity, flock_info) in query.iter_mut() {
        let center = flock_info.average_position();
        let delta = center - position.vec;
        let delta_len = delta.length();
        if delta_len <= EPSILON {
            continue;
        }
        let directional_force = steer_towards(rot.vec(), delta.normalize())
            .mul(behavior.cohesion_force)
            .min(behavior.max_cohesion_force);
        velocity.radians_per_second += directional_force;
    }
}

pub fn apply_alignment(behavior: Res<BoidBehavior>, time: Res<Time>, mut query: Query<(&Position, &Rotation, &mut RotationalVelocity, &BoidFlockInfo), With<Boid>>){
    for (position, rot, mut velocity, flock_info) in query.iter_mut() {
        let average_direction = flock_info.average_direction().angle_between(Vec2::X);
        let self_direction = rot.radians;
        let direction_delta = average_direction - self_direction;
        if direction_delta <= EPSILON {
            continue;
        }
        // rad
        let rot_force = direction_delta
            .mul(behavior.alignment_force)
            .min(behavior.max_alignment_force)
            .mul(time.delta_seconds());
        velocity.radians_per_second += rot_force;
    }
}

pub fn apply_wander(behavior: Res<BoidBehavior>, time: Res<Time>, mut query: Query<(&mut BoidSeed, &mut Rotation), With<Boid>>){
    for (mut seed, mut rot) in query.iter_mut() {
        rot.radians += seed.0.gen_range(-behavior.wander_angle_range..behavior.wander_angle_range);
    }
}