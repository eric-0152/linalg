use crate::matrix::Matrix;
use crate::polynomial::Polynomial;
use crate::solve;
use crate::eigen;

#[test]
fn lambda_polynomial() {
    let matrix = Matrix::read_txt("./src/test/_eigen/lambda_polynomial/p1.txt").unwrap();
    let answer = Polynomial::read_txt("./src/test/_eigen/lambda_polynomial/a1.txt").unwrap();
    assert_eq!(eigen::lambda_polynomial(&matrix).coeff, answer.coeff);
    
    let matrix = Matrix::read_txt("./src/test/_eigen/lambda_polynomial/p2.txt").unwrap();
    let answer = Polynomial::read_txt("./src/test/_eigen/lambda_polynomial/a2.txt").unwrap();
    assert_eq!(eigen::lambda_polynomial(&matrix).coeff, answer.coeff);
}

