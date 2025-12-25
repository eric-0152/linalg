use num_complex::Complex64;
use crate::matrix::Matrix;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub degree: usize,
    pub coeff: Vec<Complex64>,
}

impl Add<f64> for &Polynomial {
    type Output = Polynomial;
    fn add(self, float: f64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] += float;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<&Polynomial> for f64 {
    type Output = Polynomial;
    fn add(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] += self;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<Complex64> for &Polynomial {
    type Output = Polynomial;
    fn add(self, complex: Complex64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] += complex;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<&Polynomial> for Complex64 {
    type Output = Polynomial;
    fn add(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] += self;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    fn add(self, polynomial: &Polynomial) -> Self::Output {
        let (mut p1, p2): (Polynomial, Polynomial) = Polynomial::to_same_size(self, polynomial);
        for d in 0..p1.coeff.len() {
            p1.coeff[d] += p2.coeff[d]
        }
        
        p1.degree = p1.coeff.len() - 1;
        p1.remove_redundant()
    }
}

impl Sub<f64> for &Polynomial {
    type Output = Polynomial;
    fn sub(self, float: f64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] -= float;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<&Polynomial> for f64 {
    type Output = Polynomial;
    fn sub(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] -= self;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<Complex64> for &Polynomial {
    type Output = Polynomial;
    fn sub(self, complex: Complex64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] -= complex;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<&Polynomial> for Complex64 {
    type Output = Polynomial;
    fn sub(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] -= self;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    fn sub(self, polynomial: &Polynomial) -> Self::Output {
        let (mut p1, p2): (Polynomial, Polynomial) = Polynomial::to_same_size(self, polynomial);
        for d in 0..p1.coeff.len() {
            p1.coeff[d] -= p2.coeff[d]
        }
        
        p1.degree = p1.coeff.len() - 1;
        p1.remove_redundant()
    }
}

impl Mul<f64> for &Polynomial {
    type Output = Polynomial;
    fn mul(self, float: f64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] *= float;
        }
        
        result_polynomial.remove_redundant()
    }
}
impl Mul<&Polynomial> for f64 {
    type Output = Polynomial;
    fn mul(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] *= self;
        }
        
        result_polynomial.remove_redundant()
    }
}
impl Mul<Complex64> for &Polynomial {
    type Output = Polynomial;
    fn mul(self, complex: Complex64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] *= complex;
        }
        
        result_polynomial.remove_redundant()
    }
}
impl Mul<&Polynomial> for Complex64 {
    type Output = Polynomial;
    fn mul(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] *= self;
        }
        
        result_polynomial.remove_redundant()
    }
}
impl Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    fn mul(self, polynomial: &Polynomial) -> Self::Output {
        let p1: Polynomial = self.clone();
        let p2: Polynomial = polynomial.clone();

        if p1.degree == 0 {
            if p1.coeff.len() == 0 {
                return Polynomial::new(&vec![Complex64::ZERO]);
            } else {
                return p1.coeff[0] * &p2;
            }    
        } else if p2.degree == 0 {
            if p2.coeff.len() == 0 {
                return Polynomial::new(&vec![Complex64::ZERO]);
            } else {
                return p2.coeff[0] * &p1;
            }
        }
        

        let mut result_polynomial: Polynomial = Polynomial::new(&vec![Complex64::ZERO]);
        while result_polynomial.degree < p1.degree + p2.degree {
            result_polynomial.coeff.push(Complex64::ZERO);
            result_polynomial.degree += 1;
        }
        for i in 0..p1.coeff.len() {
            for j in 0..p2.coeff.len() {
                result_polynomial.coeff[i + j] += p1.coeff[i] * p2.coeff[j];
            }
        }

        result_polynomial.remove_redundant()
    }
}
impl Div<f64> for &Polynomial {
    type Output = Polynomial;
    fn div(self, float: f64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] /= float;
        }
        
        result_polynomial.remove_redundant()
    }
}
impl Div<Complex64> for &Polynomial {
    type Output = Polynomial;
    fn div(self, complex: Complex64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] /= complex;
        }
        
        result_polynomial
    }
}


impl Polynomial {
    pub fn new(coefficients: &Vec<Complex64>) -> Polynomial {
        if coefficients.len() == 0 {
            return Polynomial::new(&vec![Complex64::ZERO]);
        }
        
        let degree: usize = coefficients.len() - 1;
        Polynomial { degree, coeff: coefficients.clone() }
    }
    pub fn zero() -> Polynomial {
        Polynomial { degree: 0, coeff: vec![Complex64::ZERO] }
    }    
    pub fn one() -> Polynomial {
        Polynomial { degree: 0, coeff: vec![Complex64::ONE] }
    }
    
    pub fn display(self: &Self) {
        let mut show_im = false;
        for e in 0..self.coeff.len() {
            if self.coeff[e].im != 0.0 {
                show_im = true;
                break;
            }
        }
        
        print!("[");
        if show_im {
            for e in 0..self.coeff.len() {
                if self.coeff[e].im >= 0.0 {
                    print!("{}", format!("{:>11?} {:>11} j", self.coeff[e].re, format!("+ {:<?}", self.coeff[e].im.abs())));
                } else {
                    print!("{}", format!("{:>11?} {:>11} j", self.coeff[e].re, format!("- {:<?}", self.coeff[e].im.abs())));
                }
                
                if e != (self.coeff.len() - 1) {
                    print!(",");
                }
            }
        } else {
            for e in 0..self.coeff.len() {
                print!("{}", format!("{:>11?}", self.coeff[e].re));
                if e != (self.coeff.len() - 1) {
                    print!(",")
                }
            }
        }
        println!("], degree: {}", self.degree);
    }
    
    pub fn to_same_size(polynomial1: &Polynomial, polynomial2: &Polynomial) -> (Polynomial, Polynomial) {    
        let p1 = polynomial1.remove_redundant();
        let p2 = polynomial2.remove_redundant();
        if p1.coeff.len() < p2.coeff.len() {
            let mut result_polynomial1: Polynomial = p1.clone();
            while result_polynomial1.coeff.len() < p2.coeff.len() {
                result_polynomial1.coeff.push(Complex64::ZERO);
            }
            
            return (result_polynomial1, p2.clone())
        } else if p1.coeff.len() > p2.coeff.len() {
            let mut result_polynomial2: Polynomial = p2.clone();
            while result_polynomial2.coeff.len() < p1.coeff.len() {
                result_polynomial2.coeff.push(Complex64::ZERO);
            }
            
            return (polynomial1.clone(), result_polynomial2)
        }
        
        (p1, p2)
    }
    
    pub fn remove_redundant(self: &Self) -> Polynomial {
        let mut result_polynomial: Polynomial = self.clone();
        let mut row_index = result_polynomial.coeff.len() - 1;
        while row_index > 0 && result_polynomial.coeff[row_index] == Complex64::ZERO {
            result_polynomial.coeff.pop();
            row_index -= 1;
        }
    
        result_polynomial
    }

}


pub fn newton_raphson(polynomial: &Matrix) -> Matrix {
    polynomial.clone()
}

