use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::intersection::color_at;
use crate::world::World;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn render(camera: Arc<Camera>, world: Arc<World>) -> Canvas {
    let canvas = Arc::new(Mutex::new(Canvas::new(camera.hsize, camera.vsize)));

    assert_ne!(
        world.light_sources.len(),
        0,
        "World doesn't have any lights"
    );

    for y in 0..camera.vsize - 1 {
        GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
        let canvas = canvas.clone();
        let world = world.clone();
        let camera = camera.clone();
        thread::spawn(move || {
            for x in 0..camera.hsize - 1 {
                let ray = camera.ray_for_pixel(x, y);
                let color = color_at(world.clone(), ray);
                canvas.lock().unwrap().write_pixel(x, y, color);
            }
            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
    }

    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }

    Arc::try_unwrap(canvas).unwrap().into_inner().unwrap()
}
