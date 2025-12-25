#[path = "basic/_decomposition.rs"]
mod decomposition;
#[path = "basic/_eigen.rs"]
mod eigen;
#[path = "basic/_io.rs"]
mod io;
#[path = "basic/_matrix.rs"]
mod matrix;
#[path = "basic/_vector.rs"]
mod vector;
#[path = "basic/_process.rs"]
mod process;
#[path = "basic/_solve.rs"]
mod solve;
#[path = "basic/_transform.rs"]
mod transform;
#[path = "basic/_polynomial.rs"]
pub mod polynomial;

#[path = "optimize/_mcmc.rs"]
mod mcmc;
#[path = "optimize/_preprocessing.rs"]
mod preprocessing;
#[path = "optimize/_regression.rs"]
mod regression;

mod test;
use matrix::Matrix;
use num_complex::Complex64;
use polynomial::Polynomial;
use rand_distr::num_traits::ConstZero;

fn main() {    
    let mut mat = Matrix::random_matrix(5, 5, -100.0, 100.0, true).round(5);
    let mut t: Matrix = Matrix::random_matrix(3, 3, -9.0, 9.0, false).round(0);
 
    to_polynomial![[1,4,5,"6 + 3j"]];

    // polynomial::newton_raphson(&Polynomial::new(&vec![Complex64::ONE, Complex64::ZERO, Complex64::ONE]));
}

