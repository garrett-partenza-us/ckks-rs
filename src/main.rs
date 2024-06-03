use ckks_rs::poly::univariate::Polynomial;

fn main() {
    let poly1 = Polynomial::new(vec![-4, 0, -2, 1]);
    let poly2 = Polynomial::new(vec![-3, 1]);
    let (q, r) = poly1 / poly2;
    println!("Q: {}, R: {}", q, r);
}
