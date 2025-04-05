use raytracer::{
    bin_utils::{tick, Environment, Projectile},
    tuple::Tuple,
};

pub fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 5.0, 0.0).normalize(),
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut n_ticks = 0;

    while p.position.y > 0.0 {
        p = tick(&e, &p);
        println!("{:?}", p);
        n_ticks += 1;
    }
    println!("Done after {:?} ticks", n_ticks)
}
