use raytracer::{bin_utils::{Projectile, Environment, tick}, tuple::Tuple, colors::Color, canvas::Canvas};

fn draw_trajectory(canvas: &mut Canvas, projectile: &Projectile) {
    let trajectory_color = Color::new(0.8, 0.3, 0.3);

    let x = projectile.position.x.round() as i32;
    let y = projectile.position.y.round() as i32;
    canvas.write_pixel(x, y, trajectory_color);
}

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


    let mut canvas = Canvas::new(900, 550);

    while p.position.y > 0.0 {
        p = tick(&e, &p);
        n_ticks += 1;

        draw_trajectory(&mut canvas, &p);
    }
    println!("Done after {:?} ticks", n_ticks)
}