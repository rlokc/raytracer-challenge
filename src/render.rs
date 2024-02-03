use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::intersection::color_at;
use crate::world::World;

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut canvas = Canvas::new(camera.hsize, camera.vsize);

    for y in  0..camera.vsize-1 {
        println!("Got through {}", y);
        for x in 0..camera.hsize-1 {
            let ray = camera.ray_for_pixel(x, y);
            let color = color_at(world, ray);
            canvas.write_pixel(x, y, color);
        }
    }

    canvas
}