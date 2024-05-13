extern crate num;
extern crate nalgebra as nai

use std::f64::consts::PI;
use num::Complex;
use nalgebra::{DVector, DMatrix};

const M: usize = 8;

fn main() {
    
    println!("Welcome to CKKS-RS!");
 
    let N: usize = M/2;

    let data = vec![1.0, 2.0, 3.0, 4.0];

    sigma(&data);

}


fn vandermonde() -> Vec<Vec<Complex<f64>>> {
    let xi: Complex<f64> = ( 2 as f64 * PI * ( Complex::new(1.0, 1.0) / M as f64));
    let mut vand_mat: Vec<Vec<Complex<f64>>> = Vec::new();
    let N: usize = M/2;
    for r in 0..N {
        let root: Complex<f64> = xi * (2.0 * r as f64 + 1 as f64);
        let mut row: Vec<Complex<f64>> = Vec::new();
        for c in 0..N {
            row.push(c as f64 * root);
        }
        vand_mat.push(row);
    }
    vand_mat
}

fn sigma(data: &Vec<f64>) -> () {
    let vandermonde_matrix = DMatrix::from_fn(4, 4, |r, c| vandermonde()[r][c]);
    let y: Vec<Complex<f64>> = DVector::from_vec(
        data.iter().map(|&val| Complex::new(val, 0.0)).collect();
        )
    let b = vandermonde_matrix.qr().solve(&y);

}

