use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub transform: Matrix,

    half_width: f64,
    half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Camera {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = half_width * 2.0 / hsize as f64;

        Camera {
            hsize,
            vsize,
            fov,
            transform: Matrix::identity_matrix(4),
            half_height,
            half_width,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // Offset from the edge of the canvas to the pixel's center
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // Untransformed coords of the pixel in world space (+x is to the left because camera looks towards -z)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // transform the canvas point and the origin, and then compute the ray's direction vector
        let inv = self.transform.invert().unwrap();
        let pixel = inv.tuple_mul(&Tuple::point(world_x, world_y, -1.0));
        let origin = inv.tuple_mul(&Tuple::point(0.0, 0.0, 0.0));
        let direction = pixel.sub(origin).normalize();

        Ray::new(origin, direction)
    }
}
