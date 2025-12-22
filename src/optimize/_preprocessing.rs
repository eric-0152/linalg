use crate::{matrix::Matrix, vector::Vector};

pub fn normalize(data: &Vector) -> Vector {
    if data.size == 0 {
        return data.clone();
    }

    let mut min: f64 = data.entries[0];
    let mut max: f64 = data.entries[0];
    for e in 0..data.size {
        if data.entries[e] < min {
            min = data.entries[e];
        } else if data.entries[e] > max {
            max = data.entries[e];
        }
    }

    let mut norm_data: Vector = data.clone();
    let denominator: f64 = max - min;
    for e in 0..norm_data.size {
        norm_data.entries[e] = (norm_data.entries[e] - min) / denominator;
    }

    norm_data
}

/// ## NEED TO FIX
pub fn principle_component_analysis(matrix: &Matrix, dimension: usize) -> Result<Matrix, String> {
    if dimension > matrix.row {
        return Err("Input Error: Parameter dimension cannot be greater than matrix's row".to_string());
    }
    
    let mut row_mean: Matrix = Matrix::zeros(matrix.row, 1);
    for r in 0..matrix.row {
        let mut sum: f64 = 0.0;
        for c in 0..matrix.col {
            sum += matrix.entries[r][c];
        }
        row_mean.entries[r][0] = sum / matrix.col as f64;
    }
    let mean_matrix: Matrix = row_mean.multiply_Matrix(&Matrix::ones(1, matrix.col)).unwrap();
    let residual_matrix: Matrix = matrix.substract_Matrix(&mean_matrix).unwrap();
    let (u, _, _) = residual_matrix.singular_value_decomposition().unwrap();
    let mut principle_component: Matrix = Matrix::zeros(0, 0);
    for d in 0..dimension {
        if d == u.col {break;}
        principle_component = principle_component.append_Vector(&u.get_column_vector(d).unwrap(), 0).unwrap();
    }

    let result_matrix: Matrix = principle_component.multiply_Matrix(&mean_matrix).unwrap();
    Ok(result_matrix)
}