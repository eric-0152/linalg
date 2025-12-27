#![allow(dead_code, unused)]
#![allow(warnings)]


mod test;
use math_tool::{decomposition, eigen, io, matrix::Matrix, polynomial::Polynomial, multipoly::MultiPoly, process, regression, solve, transform, vector::Vector, *};
use num_complex::Complex64;
use rand_distr::num_traits::ConstOne;
fn main() {    
    let mut mat = Matrix::random_matrix(4, 4, -100.0, 100.0, false).round(0);
    let mut t: Matrix = Matrix::random_matrix(3, 3, -9.0, 9.0, false).round(0);
    
    let matrix = Matrix::read_txt("./src/test/_eigen/eigenvalue/p2.txt").unwrap();
    let answer = Vector::read_txt("./src/test/_eigen/eigenvalue/a2.txt").unwrap();
    let mut conj_answer = answer.clone();
    conj_answer.entries[1].im = -conj_answer.entries[1].im;
    conj_answer.entries[2].im = -conj_answer.entries[2].im;
    let eigen = eigen::eigenvalue(&matrix).unwrap();
    eigen.round(8).display();
    answer.display();
    conj_answer.display();
    assert!(eigen.round(5).entries == answer.round(5).entries || eigen.round(5).entries == conj_answer.round(5).entries);
 
     
     
    // let t = mat.svd().unwrap();
    // t.1.display();
    // (&(&(&t.0.round(5) * &t.1.round(5)) * &t.2.round(5)) - &mat).round(5).display();
}

