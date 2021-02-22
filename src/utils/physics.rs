use crate::config;
use crate::human::Human;
use crowdstress_common::primitives::Point;
use crowdstress_common::vector::Vector;

pub fn f1(human: &Human, target: &Point) -> Vector {
    let vector_to_target = Vector::from_points(&human.coords, target).normalize();
    let v = vector_to_target.product(config::V);
    let u = vector_to_target.product(config::U);
    u.subtract(&v).product(config::HUMAN_MASS / config::DELTA_T)
}

pub fn f2(vector_to_human: Vector) -> Vector {
    let n = vector_to_human.normalize();
    let distance_to_human =
        (vector_to_human.get_length() - 2.0 * config::R) / config::SCALING_FACTOR;
    n.product(config::A * (distance_to_human / config::B).exp())
}

pub fn f2w(vector_to_wall: Vector) -> Vector {
    let n = vector_to_wall.normalize();
    let distance_to_wall = (vector_to_wall.get_length() - config::R) / config::SCALING_FACTOR;
    if distance_to_wall < 0.0 {
        Vector::origin()
    } else {
        n.product(config::A_W * (distance_to_wall / config::B_W).exp())
    }
}
