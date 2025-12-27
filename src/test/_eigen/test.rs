use math_tool::{matrix::Matrix, polynomial::Polynomial, vector::Vector, *};

#[test]
fn lambda_polynomial() {
    let matrix = Matrix::read_txt("./src/test/_eigen/lambda_polynomial/p1.txt").unwrap();
    let answer = Polynomial::read_txt("./src/test/_eigen/lambda_polynomial/a1.txt").unwrap();
    assert_eq!(eigen::lambda_polynomial(&matrix).coeff, answer.coeff);

    let matrix = Matrix::read_txt("./src/test/_eigen/lambda_polynomial/p2.txt").unwrap();
    let answer = Polynomial::read_txt("./src/test/_eigen/lambda_polynomial/a2.txt").unwrap();
    assert_eq!(eigen::lambda_polynomial(&matrix).coeff, answer.coeff);
}

#[test]
fn eigenvalue() {
    let matrix = Matrix::read_txt("./src/test/_eigen/eigenvalue/p1.txt").unwrap();
    let answer = Vector::read_txt("./src/test/_eigen/eigenvalue/a1.txt").unwrap();
    assert_eq!(eigen::eigenvalue(&matrix).unwrap().round(8).entries, answer.round(8).entries);

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
}
