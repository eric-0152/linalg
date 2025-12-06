use crate::matrix::Matrix;

impl Matrix {
    // pub fn get_eigenvalue_qr(self: &Self) -> Result<Matrix, String> {
    //     const THERSHOLD: f64 = 0.0001;
    //     let mut qr: (Matrix, Matrix) = self.qr_decomposition();
    //     let mut similar: Matrix = qr.1.multiply_Matrix(&qr.0).unwrap();
    //     let mut old_similar: Matrix = similar.copy();
    //     let mut diff: f64 = 1.0;

    //     while diff > 0.9 {
    //         qr = similar.qr_decomposition();
    //         similar = qr.1.multiply_Matrix(&qr.0).unwrap();
    //         diff = (similar.trace() - old_similar.trace()).abs();
    //         similar.display();
    //         old_similar.display();
    //         println!("{}\n\n",diff);

    //         old_similar = similar.copy();
    //         similar.round(2);
    //     }

    //     Ok(similar)
    // }
}
