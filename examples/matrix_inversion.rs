extern crate nalgebra as na;

use na::Matrix4;

fn main() {
    let invertible = Matrix4::new(
        5., 6., 6., 8., 2., 2., 2., 8., 6., 6., 2., 8., 2., 3., 6., 7.,
    );

    let not_invertible = Matrix4::zeros();

    for matrix in [invertible, not_invertible] {
        if matrix.is_invertible() {
            println!("{:?} is invertible.", matrix);
            println!("Inverse: {:?}", matrix.try_inverse().unwrap());
        } else {
            println!("{:?} is not invertible.", matrix);
        }
    }
}
