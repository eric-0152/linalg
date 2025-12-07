use crate::matrix::Matrix;
use rand::Rng;

#[derive(Clone)]
pub struct Vector {
    pub entries: Vec<f64>,
    pub size: usize,
}

impl Vector {
    pub fn from_vec(vector: &Vec<f64>) -> Vector {
        Vector {
            entries: vector.clone(),
            size: vector.len(),
        }
    }

    /// Return the vector that round to the digit after decimal point.
    pub fn round(self: &Self, digit: u32) -> Vector{
        let scale: f64 = 10_i32.pow(digit as u32) as f64;
        let mut result_vector = Self::zeros(self.size);

        for s in 0..self.size {
            result_vector.entries[s] = (scale * self.entries[s]).round();
            
            if result_vector.entries[s] >= 1.0 || result_vector.entries[s] <= -1.0 {
                result_vector.entries[s] /= scale;
            } else if result_vector.entries[s].is_nan() {
                    continue;
            } else {
                result_vector.entries[s] = 0.0;
            }
        }

        result_vector
    }

    pub fn display(self: &Self) {
        println!("Vector:  {:8?}, size: {}", self.entries, self.size);
    }

    /// Return a vector contains all zero entries with size m.
    pub fn zeros(m: usize) -> Vector {
        Vector {
            entries: vec![0.0; m],
            size: m,
        }
    }

    /// Return a vector contains all one entries with size m.
    pub fn ones(m: usize) -> Vector {
        Vector {
            entries: vec![1.0; m],
            size: m,
        }
    }

    /// Return a vector contains a arithmetic sequence.
    pub fn arithmetic_sequence(start: f64, end: f64, step: f64) -> Vector {
        let mut result_vector = vec![start];
        let mut current_element = start;
        let mut size: usize = 0;
        while current_element < end {
            current_element += step;
            result_vector.push(current_element);
            size += 1;
        }

        Vector {
            entries: result_vector,
            size: size,
        }
    }

    pub fn random_vector(m: usize, min: f64, max: f64) -> Vector {
        let mut generator: rand::prelude::ThreadRng = rand::rng();
        let mut entries: Vec<f64> = Vec::new(); 
        for _ in 0..m {
            entries.push(generator.random_range(min..max));
        } 
        Vector {
            entries: entries,
            size: m,
        }
    }

    pub fn clone(self: &Self) -> Vector {
        Vector {
            entries: self.entries.clone(),
            size: self.size,
        }
    }

    pub fn reverse(self: &Self) -> Vector {
        let mut new_entries = self.entries.clone();
        new_entries.reverse();

        Vector {entries: new_entries, size: self.size}
    }

    pub fn transpose(self: &Self) -> Matrix {
        let mut result_matrix = Matrix::zeros(self.size, 1);
        for r in 0..result_matrix.row {
            result_matrix.entries[r][0] = self.entries[r];
        }

        result_matrix
    }

    pub fn swap_element(self: &Self, a: usize, b: usize) -> Result<Vector, String> {
        if a >= self.size || b >= self.size {
            return Err("Input Error: The input a or b is out of bound.".to_string());
        }

        let mut result_Vector = self.clone();
        result_Vector.entries[a] = self.entries[b];
        result_Vector.entries[b] = self.entries[a];

        Ok(result_Vector)
    }

    /// Add two vector element-wise.
    pub fn add_Vector(self: &Self, vector: Vector) -> Result<Vector, String> {
        if self.size != vector.size {
            return Err("Input Error: The size of input vector does not match.".to_string());
        }

        let mut result_vector = self.clone();
        for s in 0..self.size {
            result_vector.entries[s] += vector.entries[s];
        }

        Ok(result_vector)
    }

    /// Substract two vector element-wise.
    pub fn substract_Vector(self: &Self, vector: Vector) -> Result<Vector, String> {
        if self.size != vector.size {
            return Err("Input Error: The size of input vector does not match.".to_string());
        }

        let mut result_vector = self.clone();
        for s in 0..self.size {
            result_vector.entries[s] -= vector.entries[s];
        }

        Ok(result_vector)
    }

    pub fn multiply_scalar(self: &Self, scalar: f64) -> Vector {
        let mut result_vector = Self::zeros(self.size);
        for s in 0..self.size {
            result_vector.entries[s] = scalar * self.entries[s];
        }

        result_vector
    }

    pub fn inner_product(self: &Vector, vector: &Vector) -> Result<f64, String> {
        if self.size != vector.size {
            return Err("Input Error: The size of two vectors do not match.".to_string());
        }

        let mut result: f64 = 0.0;
        for e in 0..self.size {
            result += self.entries[e] * vector.entries[e];
        }

        Ok(result)
    }

    /// Compute cross peoduct for two 3-dimension vectors.
    pub fn cross_product(self: &Vector, vector: &Vector) -> Result<Vector, String> {
        if self.size != vector.size || self.size != 3 {
            return Err("Input Error: The size of input vectors should be 3.".to_string());
        }

        let mut result_vector: Vector = Self::zeros(3);
        let mut matrix: Matrix =
            Matrix::from_double_vec(&vec![self.entries.clone(), vector.entries.clone()]);
        for d in 0..3 {
            result_vector.entries[d] = matrix.remove_col(d).unwrap().determinant().unwrap();
        }

        Ok(result_vector)
    }

    pub fn euclidean_distance(self: &Self) -> f64 {
        let mut disatnce: f64 = 0.0;

        for s in 0..self.size {
            disatnce += self.entries[s] * self.entries[s];
        }

        disatnce.sqrt()
    }
}
