extern crate nalgebra as na; // Linear algebra
extern crate num; // Complex numbers

use nalgebra::DMatrix; 
use num::Complex;
use std::f64::consts::PI;

const M: usize = 8;

fn main() {
    println!("Welcome to CKKS-RS!");

    let data = vec![1.0, 2.0, 3.0, 4.0];

    println!("Plain data: {:?}", data);

    let encoded = sigma_inverse(&data);

    println!("Encoded: {:?}", encoded);
}

fn vandermonde() -> Vec<Vec<Complex<f64>>> {
    let xi: Complex<f64> = 2 as f64 * PI * (Complex::new(1.0, 1.0) / M as f64);
    let mut vand_mat: Vec<Vec<Complex<f64>>> = Vec::new();
    let n: usize = M / 2;
    for r in 0..n {
        let root: Complex<f64> = xi.powf(2.0 * r as f64 + 1 as f64);
        let mut row: Vec<Complex<f64>> = Vec::new();
        for c in 0..n {
            row.push(root.powf(c as f64));
        }
        vand_mat.push(row);
    }
    vand_mat
}

fn sigma_inverse(data: &Vec<f64>) -> DMatrix<Complex<f64>> {
    let vandermonde_matrix = vandermonde();
    let vandermonde_matrix = DMatrix::from_fn(4, 4, |r, c| vandermonde_matrix[r][c]);
    let y = DMatrix::from_fn(4, 1, |r, _| Complex::new(data[r], 0.0));
    let b = vandermonde_matrix
        .qr()
        .solve(&y)
        .expect("Failed to find matrix inverse.");
    b
}

fn sigma(encoded: &DMatrix<Complex<f64>>) -> () {
    // TODO
}
