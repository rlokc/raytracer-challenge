use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

#[derive(Debug)]
pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

pub fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position.add(proj.velocity);
    let velocity = proj.velocity.add(env.gravity).add(env.wind);
    Projectile { position, velocity }
}