use num_complex::Complex64;
use crate::vector::Vector;
use std::ops::{Add, Div, Mul, Sub};
use crate::io;

/// Coefficient of the polynomial is start from constant
#[derive(Clone, Debug)]
pub struct Polynomial {
    pub degree: usize,
    pub coeff: Vec<Complex64>,
}

#[macro_export]
macro_rules! to_polynomial {
    (
        [$( $e:expr),*] 
    ) => {{
        let mut elements = Vec::new();
        $(
            elements.push(io::_parse_str(format!("{}", $e).as_str()).unwrap());
        )*

        polynomial::Polynomial::new(&elements)
    }};
}

impl Add<f64> for &Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, float: f64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] += float;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<&Polynomial> for f64 {
    type Output = Polynomial;
    #[inline]
    fn add(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] += self;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<Complex64> for &Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, complex: Complex64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] += complex;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<&Polynomial> for Complex64 {
    type Output = Polynomial;
    #[inline]
    fn add(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] += self;
        
        result_polynomial.remove_redundant()
    }
}
impl Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    #[inline]
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
    #[inline]
    fn sub(self, float: f64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] -= float;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<&Polynomial> for f64 {
    type Output = Polynomial;
    #[inline]
    fn sub(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] -= self;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<Complex64> for &Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, complex: Complex64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        result_polynomial.coeff[0] -= complex;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<&Polynomial> for Complex64 {
    type Output = Polynomial;
    #[inline]
    fn sub(self, polynomial: &Polynomial) -> Self::Output {
        let mut result_polynomial: Polynomial = polynomial.clone();
        result_polynomial.coeff[0] -= self;
        
        result_polynomial.remove_redundant()
    }
}
impl Sub<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    #[inline]
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
    #[inline]
    fn mul(self, scalar: f64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] *= scalar;
        }
        
        result_polynomial.remove_redundant()
    }
}
impl Mul<&Polynomial> for f64 {
    type Output = Polynomial;
    #[inline]
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
    #[inline]
    fn mul(self, scalar: Complex64) -> Self::Output {
        let mut result_polynomial: Polynomial = self.clone();
        for d in 0..result_polynomial.coeff.len() {
            result_polynomial.coeff[d] *= scalar;
        }
        
        result_polynomial.remove_redundant()
    }
}
impl Mul<&Polynomial> for Complex64 {
    type Output = Polynomial;
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
        Polynomial { degree, coeff: coefficients.clone() }.remove_redundant()
    }
    /// Return a polynomial with m 0.
    pub fn zero() -> Polynomial {
        Polynomial { degree: 0, coeff: vec![Complex64::ZERO] }
    }    
    /// Return a polynomial with m 1.
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
    
    #[inline]
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
    
    #[inline]
    pub fn remove_redundant(self: &Self) -> Polynomial {
        let mut result_polynomial: Polynomial = self.clone();
        let mut row_index: usize = result_polynomial.coeff.len() - 1;
        while row_index > 0 && result_polynomial.coeff[row_index] == Complex64::ZERO {
            result_polynomial.coeff.pop();
            row_index -= 1;
        }
        
        result_polynomial.degree = result_polynomial.coeff.len() - 1;
        result_polynomial
    }
    
    #[inline]
    /// Return a tuple (quotient, remainder)
    pub fn divide_by(self: &Self, divider: &Polynomial) -> (Polynomial, Polynomial) {
        let divider = divider.remove_redundant();
        let mut remainder: Polynomial = self.clone();
        let mut quotient: Polynomial = Polynomial::zero();
        let degree_one = Polynomial::new(&vec![Complex64::ZERO, Complex64::ONE]);
        let mut degree_diff: usize = remainder.degree - divider.degree;
        loop {
            let coefficient = remainder.coeff[remainder.degree] / divider.coeff[divider.degree];
            for d in 0..(divider.degree + 1) {
                remainder.coeff[d + degree_diff] -= coefficient * divider.coeff[d];
            }
            remainder = remainder.remove_redundant();
            quotient = &(&quotient * &degree_one) + &Polynomial::new(&vec![coefficient]);
            quotient.degree += 1;
            if degree_diff == 0 {
                break;
            } else {
                degree_diff -= 1;
            }
        }
        
        (quotient, remainder)
    }
    
    #[inline]
    pub fn differentiate(self: &Self) -> Polynomial {
        let mut result_polynomial: Polynomial = self.clone();
        for e in 0..self.coeff.len() - 1 {
            result_polynomial.coeff[e] = (e + 1) as f64 * self.coeff[e + 1];
        }
    
        result_polynomial
    }
}





/// The coeff stores (scalar, [degree of variables])
#[derive(Debug, Clone)]
pub struct MultiVariablePolynomial {
    variables_names: Vec<String>,
    pub coeff: Vec<(Complex64, Vec<f64>)>,
}

impl Mul<f64> for &MultiVariablePolynomial {
    type Output = MultiVariablePolynomial;

    fn mul(self, scalar: f64) -> Self::Output {
        let mut result_poly = self.clone();
        for d in 0..result_poly.coeff.len() {
            result_poly.coeff[d].0 *= scalar;
        }
        result_poly
    }
}
impl Mul<Complex64> for &MultiVariablePolynomial {
    type Output = MultiVariablePolynomial;

    fn mul(self, scalar: Complex64) -> Self::Output {
        let mut result_poly = self.clone();
        for d in 0..result_poly.coeff.len() {
            result_poly.coeff[d].0 *= scalar;
        }
        result_poly
    }
}
impl Mul<&MultiVariablePolynomial> for f64 {
    type Output = MultiVariablePolynomial;

    fn mul(self, polynomial: &MultiVariablePolynomial) -> Self::Output {
        let mut result_poly = polynomial.clone();
        for d in 0..result_poly.coeff.len() {
            result_poly.coeff[d].0 *= self;
        }
        result_poly
    }
}

impl Mul<&MultiVariablePolynomial> for Complex64 {
    type Output = MultiVariablePolynomial;

    fn mul(self, polynomial: &MultiVariablePolynomial) -> Self::Output {
        let mut result_poly = polynomial.clone();
        for d in 0..result_poly.coeff.len() {
            result_poly.coeff[d].0 *= self;
        }
        result_poly
    }
}

impl MultiVariablePolynomial {
    pub fn combination(n: u32, r: u32) -> u32 {
        if r > n {
            return 0;
        }
        
        let mut result = 1;
        for i in 0..r {
            result *= n - i;
            result /= i + 1;
        }
        
        result
    }
    pub fn new(variables_names: Vec<String>, coeff: Vec<(Complex64, Vec<f64>)>) -> Result<MultiVariablePolynomial, String> {
        for d in 0..coeff.len() {
            if variables_names.len() != coeff[d].1.len() {
                return Err("Input Error: The number of names and coefficients do not match.".to_string());
            }            
        }
        
        Ok(MultiVariablePolynomial { variables_names, coeff })
    }
    pub fn remove_redundant(self: &Self) -> MultiVariablePolynomial {
        let mut result_poly = self.clone();
        for d in (0..result_poly.coeff.len()).rev() {
            if result_poly.coeff[d].0 == Complex64::ZERO {
                result_poly.coeff.remove(d);
            }
        }
        
        result_poly
    }
    pub fn newton_raphson_poly(polynomial: Polynomial) -> MultiVariablePolynomial {
        let mut coefficient = vec![(polynomial.coeff[0], vec![0.0, 0.0])];
        for d in 1..(polynomial.degree + 1) {
            coefficient.push((polynomial.coeff[d], vec![1.0, 1.0]));
        }
        
        MultiVariablePolynomial { variables_names: vec!["x".to_string(), "y".to_string()], coeff: coefficient }
    }
}




pub fn newton_raphson(polynomial: &Polynomial) -> Result<Vector, String> {
    if polynomial.degree == 0 {
        return Err("Input Error: The input polynomial should be at least degree of 1.".to_string());
    }
    
    // To complex form of polynomial
    let mut powered_complex_x = Complex64::new(1.0, 1.0);
    let mut complex_polynomial = Polynomial::new(&vec![polynomial.coeff[0]]);
    for d in 1..(polynomial.degree + 1) {
        complex_polynomial.coeff.push(polynomial.coeff[d] * &powered_complex_x);
        // complex_polynomial = &complex_polynomial + (polynomial.coeff[d] * &powered_complex_x);
        powered_complex_x = &powered_complex_x * Complex64::new(1.0, 1.0);
    complex_polynomial.display();
    }
    
    let mut real_polynomial = complex_polynomial;
    let mut img_polynomial = real_polynomial.clone();
    for d in 0..(real_polynomial.degree + 1) {
        real_polynomial.coeff[d].im = 0.0;
        img_polynomial.coeff[d].re = 0.0;
    }
    
    real_polynomial.display();
    img_polynomial.display();
    
    Ok(Vector::ones(5))
}


