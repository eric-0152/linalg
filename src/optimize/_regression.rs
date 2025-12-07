use crate::matrix::Matrix;
use crate::vector::Vector;


/// Given a matrix ***A*** and a vecor of answer ***y***, return a vector ***x*** which |***Ax*** - ***b***| is minimized.
/// 
/// ### Formula : 
/// &emsp; ***x*** = (***A^T*** @ ***A***)^-1 @ ***A^T*** @ ***y***
pub fn least_squared_approximation(matrix: &Matrix, y: &Vector) -> Result<Vector, String> {
    if matrix.row != y.size {
        return Err("Input Error: The size of vector does not match.".to_string());
    }

    let transposed_matrix: Matrix = matrix.transpose();
    let result: Result<Matrix, String> = transposed_matrix.multiply_Matrix(matrix).unwrap().inverse();

    match result {
        Ok(inverse) => {
            return Ok(inverse
                .multiply_Matrix(&transposed_matrix)
                .unwrap()
                .multiply_Vector(y)
                .unwrap());
        }

        Err(error_msg) => {
            return Err(error_msg);
        }
    }
}

/// Return a Vector which contains the coefficients, the order is from constant to higest degree.
/// 
/// Given a corresponding ***x*** and answer ***y***, using a polynomial function to do the 
/// regression.
pub fn polynomial_regression(x: &Vector, y: &Vector, degree: usize) -> Result<Vector, String> {
    if degree < 0 {
        return Err("Input Error: The degree should be equal or larger than zero.".to_string());
    } else if x.size != y.size {
        return Err("Input Error: The size of x and y do not match.".to_string());
    }

    let mut matrix: Matrix = Vector::ones(x.size).transpose();
    let mut powered_x: Vector = x.clone();
    for _ in 0..degree {
        matrix = matrix.append_Vector(&powered_x, 1).unwrap();
        for s in 0..x.size {
            powered_x.entries[s] *= x.entries[s];
        }
    }

    match least_squared_approximation(&matrix, y) {
        Ok(coefficients) => {
            Ok(coefficients)
        }
        Err(error_msg) => {
            Err(error_msg)
        }
    } 
}
