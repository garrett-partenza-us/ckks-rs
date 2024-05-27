mod polynomial;

use crate::polynomial::Polynomial;

fn main() {

    let poly1 = Polynomial::new(vec![-3, 10, -5, 3]);
    let poly2 = Polynomial::new(vec![1, 3]);
    let (q, r) = poly1 / poly2;
    println!("Q: {}, R: {}", q, r);
} 
