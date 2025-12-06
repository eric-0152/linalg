use crate::matrix::{self, Matrix};
use crate::vector::Vector;

pub fn solve_upper_triangular(matrix: &Matrix, b: &Vector) -> Result<Vector, String> {
    if matrix.row != b.size {
        return Err("Input Error: The size of input matrix and vector b do not match.".to_string());
    } else if matrix.row != matrix.col {
        return Err("Input Error: The input matrix is not a square matrix.".to_string());
    } else {
        for r in 1..matrix.row {
            for c in 0..r {
                if matrix.entries[r][c] != 0.0 {
                    return Err("Input Error: The input matrix is not upper triangular.".to_string());
                }
            }
        }
    }

    let mut x_vector: Vector = Vector::zeros(b.size);
    for diag in (0..matrix.row).rev() {
        x_vector.entries[diag] = b.entries[diag] / matrix.entries[diag][diag];
        for prev in ((diag + 1)..matrix.row).rev() {
            x_vector.entries[diag] -= matrix.entries[diag][prev] * x_vector.entries[prev] / matrix.entries[diag][diag];
        }
    }
    Ok(x_vector)
}

pub fn solve_lower_triangular(matrix: &Matrix, b: &Vector) -> Result<Vector, String> {
    if matrix.row != b.size {
        return Err("Input Error: The size of input matrix and vector b do not match.".to_string());
    } else if matrix.row != matrix.col {
        return Err("Input Error: The input matrix is not a square matrix.".to_string());
    } else {
        for r in 0..matrix.row {
            for c in (r + 1)..matrix.col {
                if matrix.entries[r][c] != 0.0 {
                    return Err("Input Error: The input matrix is not lower triangular.".to_string());
                }
            }
        }
    }

    let mut x_vector: Vector = Vector::zeros(b.size);
    for diag in 0..matrix.row {
        x_vector.entries[diag] = b.entries[diag] / matrix.entries[diag][diag];
        for prev in 0..diag {
            x_vector.entries[diag] -= matrix.entries[diag][prev] * x_vector.entries[prev] / matrix.entries[diag][diag]; 
        }
    }

    Ok(x_vector)
}

/** Return the tuple contains matrix, b and permutation after Gaussian elimination.
 *  The algorithm will swap rows if needed (diagnal has 0), if the order of rows is 
    important, use swap_with_permutation() to yield the correct order.
 */
pub fn gaussian_elimination(matrix: &Matrix, b: &Vector) -> Result<(Matrix, Vector, Matrix), String> {
    if matrix.row != b.size {
        return Err("Input Error: The size of input matrix and vector b do not match.".to_string());
    }

    // Get upper triangular form.
    let mut result_matrix: Matrix = matrix.copy();
    let mut result_vector: Vector = b.copy();
    let mut permutation = Matrix::identity(matrix.row);
    for d in 0..result_matrix.col {
        // If the pivot is 0.0, swap to non zero.
        if result_matrix.entries[d][d] == 0.0 {
            for r in (d + 1)..result_matrix.row {
                if result_matrix.entries[r][d] != 0.0 {
                    result_matrix = result_matrix.swap_row(d, r).unwrap();
                    result_vector = result_vector.swap_element(d, r).unwrap();
                    permutation = permutation.swap_row(d, r).unwrap();
                }
            }
        }

        for r in (d + 1)..result_matrix.row {
            let scale: f64 = result_matrix.entries[r][d] / result_matrix.entries[d][d];
            result_vector.entries[r] -= scale * result_vector.entries[d];
            for e in 0..matrix.col {
                result_matrix.entries[r][e] -= scale * result_matrix.entries[d][e];
            }  
        }
    }

    // Pivots -> 1
    for r in 0..result_matrix.row {
        for c in r..result_matrix.col {
            if result_matrix.entries[r][c] != 0.0 {
                let scale: f64 = result_matrix.entries[r][c];
                for e in c..result_matrix.col {
                    result_matrix.entries[r][e] /= scale;
                }
                result_vector.entries[r] /= scale;

                break;
            }
        }
    }

    Ok((result_matrix, result_vector, permutation))
}

/// Return the tuple contains matrix, b and permutation after Gaussian Jordan elimination.
/// 
/// The algorithm will swap rows if needed (diagnal has 0), if the order of rows is 
/// important, use swap_with_permutation() to yield the correct order.
pub fn gaussian_jordan_elimination(matrix: &Matrix, b: &Vector) -> Result<(Matrix, Vector, Matrix), String> {    
    match gaussian_elimination(matrix, b) {
        Ok(tuple) => {
            let mut result_matrix: Matrix = tuple.0;
            let mut result_vector: Vector = tuple.1;
            let permutation: Matrix = tuple.2;

            let bound = result_matrix.row - 1;
            for c in  (0..result_matrix.col).rev() {
                let bottom_r = c.min(bound);
                for r in  (0..c.min(result_matrix.row)).rev() {
                    let scale = result_matrix.entries[r][c] / result_matrix.entries[bottom_r][c];
                    result_matrix.entries[r][c] -= scale * result_matrix.entries[bottom_r][c];
                    result_vector.entries[r] -= scale * result_vector.entries[bottom_r];
                }
            }

            Ok((result_matrix, result_vector, permutation))
        } 

        Err(err_msg) => {
            return Err(err_msg);
        }
    }

}