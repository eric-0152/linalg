#[path = "basic/_vector.rs"]
mod vector;
#[path = "basic/_matrix.rs"]
mod matrix;
#[path = "basic/_eigen.rs"]
mod eigen;
#[path = "basic/_solve.rs"]
mod solve;

#[path = "decomposition/_decomposition.rs"]
mod decomposition;
#[path = "process/_process.rs"]
mod process;

#[path = "optimize/_regression.rs"]
mod regression;

fn main() {
    let mut mat = matrix::Matrix::random_matrix(3, 3, 0.0, 9.0).round(0);
    let mut b = vector::Vector::random_vector(3, 0.0, 5.0).round(0);
    // mat.multiply_Matrix(&mat.inverse().unwrap()).unwrap();
    // mat.inverse().unwrap().multiply_Matrix(&mat).unwrap();

    // let v1 = vec![5.0, 9.0, 3.0];
    // let v2 = vec![5.0, 9.0, 4.0];
    // let v3 = vec![3.0, 6.0, 4.0];
    // let mut mat = matrix::Matrix::from_double_vec(&vec![v1, v2, v3]);
    // mat.display();
    // b.display();

    // let tu = mat.cholesky_decomposition().unwrap();
    // tu.0.round(2).display();
    // tu.1.round(2).display();
    // tu.0.multiply_Matrix(&tu.1).unwrap().round(2).display();


    // mat.qr_decomposition().unwrap().0.round(2).display();
    // println!("{}", mat.determinant().unwrap());
    // mat.adjoint().round(2).display();
    // mat.inverse().unwrap().display();

    mat.display();
    b.display();
    solve::gaussian_jordan_elimination(&mat, &b).unwrap().0.display();
    solve::gaussian_jordan_elimination(&mat, &b).unwrap().1.display();
    // solve::gaussian_jordan_elimination_test(&mat, &b).unwrap().0;
    // solve::gaussian_jordan_elimination_test(&mat, &b).unwrap().1;
    // let mat = matrix::Matrix::random_matrix(5, 5, -1.0, 1.0);
    // mat.display();
    // mat.round(1).display();
}
