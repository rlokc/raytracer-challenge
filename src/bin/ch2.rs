use raytracer::{bin_utils::{Projectile, Environment, tick}, tuple::Tuple, colors::Color, canvas::Canvas};

fn draw_trajectory(canvas: &mut Canvas, projectile: &Projectile) {
    let trajectory_color = Color::new(0.8, 0.3, 0.3);

    let x = projectile.position.x.round() as usize;
    let mut y = projectile.position.y.round() as usize;
    y = canvas.height() - y;
    canvas.write_pixel(x, y, trajectory_color);
}

pub fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize().scalar_mul(11.25),
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

    canvas.to_ppm_file("output.ppm").unwrap();

    println!("Done after {:?} ticks", n_ticks)
}