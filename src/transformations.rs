use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::identity_matrix(4).translate(x, y, z)
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::identity_matrix(4).scale(x, y, z)
}

pub fn rotate_x(rad: f64) -> Matrix {
    Matrix::identity_matrix(4).rotate_x(rad)
}

pub fn rotate_y(rad: f64) -> Matrix {
    Matrix::identity_matrix(4).rotate_y(rad)
}

pub fn rotate_z(rad: f64) -> Matrix {
    Matrix::identity_matrix(4).rotate_z(rad)
}

pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    Matrix::identity_matrix(4).shear(xy, xz, yx, yz, zx, zy)
}

/*
from: Where the eye is positioned
to: Where to point the eye
up: The vector pointing upwards
 */
pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    let forward = to.sub(from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);

    let mut orientation = Matrix::identity_matrix(4);
    orientation.vals[0][0] = left.x;
    orientation.vals[0][1] = left.y;
    orientation.vals[0][2] = left.z;

    orientation.vals[1][0] = true_up.x;
    orientation.vals[1][1] = true_up.y;
    orientation.vals[1][2] = true_up.z;

    orientation.vals[2][0] = -forward.x;
    orientation.vals[2][1] = -forward.y;
    orientation.vals[2][2] = -forward.z;

    orientation.mat_mul(&translate(-from.x, -from.y, -from.z))
}
