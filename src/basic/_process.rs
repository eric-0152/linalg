use crate::matrix::{self, Matrix};
use crate::vector::Vector;

impl Matrix {
    // / Return the matrix which contains orthonormal basis.
    pub fn gram_schmidt(self: &Self) -> Result<Matrix, String> {
        if self.row == 0 {
            return Err("Value Error: This matrix has no column.".to_string());
        }

        let mut matrix: Matrix = self.clone();
        if matrix.row < matrix.col {
            matrix = matrix.transpose();
        } 

        let mut current_col: Vector = matrix.get_column_vector(0).unwrap();
        let mut orthonormal_matrix: Matrix = current_col
            .multiply_scalar(1.0 / current_col.euclidean_distance())
            .to_Matrix(1)
            .unwrap();

        for c in 1..matrix.col {
            current_col = matrix.get_column_vector(c).unwrap();
            let mut new_orthonormal = current_col.clone();
            for pre_c in 0..c {
                let previous_orthonormal: Vector = orthonormal_matrix.get_column_vector(pre_c).unwrap();
                let dot_product: f64 = previous_orthonormal.inner_product(&current_col).unwrap();
                new_orthonormal = new_orthonormal
                    .substract_Vector(&previous_orthonormal.multiply_scalar(dot_product))
                    .unwrap();
            }

            new_orthonormal = new_orthonormal.multiply_scalar(1.0 / new_orthonormal.euclidean_distance());
            orthonormal_matrix = orthonormal_matrix.append_Vector(&new_orthonormal, 1).unwrap();
        }

        Ok(orthonormal_matrix)
    }
}
