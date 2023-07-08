use raytracer::tuple::Tuple;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position.add(proj.velocity);
    let velocity = proj.velocity.add(env.gravity).add(env.wind);
    Projectile { position, velocity }
}

pub fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 5.0, 0.0).normalize()
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0)
    };

    let mut n_ticks = 0;

    while p.position.y > 0.0 {
        p = tick(&e, &p);
        println!("{:?}", p);
        n_ticks += 1;
    }
    println!("Done after {:?} ticks", n_ticks)
}
