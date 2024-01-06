use crate::matrix::Matrix;

pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::identity_matrix(4).translate(x, y, z)
}


pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::identity_matrix(4).scale(x, y, z)
}

pub fn rotate_x(rad: f32) -> Matrix {
    Matrix::identity_matrix(4).rotate_x(rad)
}

pub fn rotate_y(rad: f32) -> Matrix {
    Matrix::identity_matrix(4).rotate_y(rad)
}

pub fn rotate_z(rad: f32) -> Matrix {
    Matrix::identity_matrix(4).rotate_z(rad)
}

pub fn shear(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
    Matrix::identity_matrix(4).shear(xy, xz, yx, yz, zx, zy)
}
