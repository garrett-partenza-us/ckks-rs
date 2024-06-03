use crate::poly::term::Term;
use num::Num;
use num::Zero;
use std::fmt;
use std::ops;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Polynomial<T> {
    terms: Vec<Term<T>>,
}

impl<T> Polynomial<T>
where
    T: Num + Clone,
{
    pub fn new(terms: Vec<T>) -> Self {
        let terms = terms
            .into_iter()
            .enumerate()
            .map(|(degree, coefficient)| Term::new(coefficient, degree))
            .collect();
        Polynomial { terms }
    }

    fn degree(&self) -> usize {
        self.terms
            .iter()
            .filter_map(|term| {
                if term.coefficient != T::zero() {
                    Some(term.degree)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    }

    fn is_zero(&self) -> bool {
        self.terms
            .iter()
            .all(|term| term.coefficient == T::zero())
    }

    fn lead_term(&self) -> Option<&Term<T>> {
        self.terms
            .iter()
            .rev()
            .find(|term| term.coefficient != T::zero())
    }
}

impl<T: fmt::Display + Zero> fmt::Display for Polynomial<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for term in &self.terms {
            if first {
                if term.coefficient.is_zero() {
                    continue;
                }
                write!(f, "{}", term)?;
                first = false;
            } else {
                if term.coefficient.is_zero() {
                    continue;
                }
                write!(f, " + {}", term)?;
            }
        }
        Ok(())
    }
}

impl<T> ops::Add<Polynomial<T>> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: Polynomial<T>) -> Polynomial<T> {
        let mut result_terms = Vec::new();
        
        // Find the maximum degree among the two polynomials
        let max_degree = self.terms.iter().chain(rhs.terms.iter())
            .map(|term| term.degree)
            .max()
            .unwrap_or(0);

        // Adding corresponding terms
        for deg in 0..=max_degree {
            let l_term = self.terms.iter().find(|term| term.degree == deg).cloned().unwrap_or(Term::new(T::zero(), deg));
            let r_term = rhs.terms.iter().find(|term| term.degree == deg).cloned().unwrap_or(Term::new(T::zero(), deg));
            result_terms.push(l_term.clone() + r_term.clone());
        }

        Polynomial { terms: result_terms }
    }
}

impl<T> ops::Mul<Term<T>> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;
    fn mul(mut self, rhs: Term<T>) -> Polynomial<T> {
        let mut terms = Vec::new();
        for d in 0..(rhs.degree) {
            terms.push(T::zero());
        }
        terms.push(rhs.coefficient);
        let result = self.clone() * Polynomial::new(terms).clone();
        result
    }
}

impl<T> ops::Sub<Term<T>> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;
    fn sub(mut self, rhs: Term<T>) -> Polynomial<T> {
        if rhs.degree > self.degree() {
            for d in (self.degree())..rhs.degree {
                self.terms.push(Term::new(T::zero(), d));
            }
        }
        self.terms[rhs.degree] = self.terms[rhs.degree].clone() - rhs.clone();
        Polynomial { terms: self.terms }
    }
}

impl<T> ops::Add<Term<T>> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;
    fn add(mut self, rhs: Term<T>) -> Polynomial<T> {
        if rhs.degree > self.degree() {
            for d in (self.degree())..rhs.degree {
                self.terms.push(Term::new(T::zero(), d));
            }
        }
        self.terms[rhs.degree] = self.terms[rhs.degree].clone() + rhs.clone();
        Polynomial { terms: self.terms }
    }
}

impl<T> ops::Sub<Polynomial<T>> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: Polynomial<T>) -> Polynomial<T> {
        let mut result_terms = Vec::new();
        
        // Find the maximum degree among the two polynomials
        let max_degree = self.terms.iter().chain(rhs.terms.iter())
            .map(|term| term.degree)
            .max()
            .unwrap_or(0);

        // Subtract corresponding terms
        for deg in 0..=max_degree {
            let l_term = self.terms.iter().find(|term| term.degree == deg).cloned().unwrap_or(Term::new(T::zero(), deg));
            let r_term = rhs.terms.iter().find(|term| term.degree == deg).cloned().unwrap_or(Term::new(T::zero(), deg));
            result_terms.push(l_term.clone() - r_term.clone());
        }

        Polynomial { terms: result_terms }
    }
}


impl<T> ops::Mul<Polynomial<T>> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: Polynomial<T>) -> Polynomial<T> {
        let mut terms: Vec<Term<T>> = (0..(self.terms.len() + rhs.terms.len() - 1))
            .enumerate()
            .map(|(degree, _)| Term::new(T::zero(), degree))
            .collect();
        for i in 0..self.terms.len() {
            for j in 0..rhs.terms.len() {
                terms[i + j] = terms[i + j].clone() + self.terms[i].clone() * rhs.terms[j].clone();
            }
        }
        Polynomial { terms: terms }
    }
}

impl<T> ops::Div<Polynomial<T>> for Polynomial<T>
where
    T: Num + Clone + std::fmt::Display,
{
    type Output = (Polynomial<T>, Polynomial<T>);

    fn div(self, rhs: Polynomial<T>) -> (Polynomial<T>, Polynomial<T>) {
        assert!(self.degree() >= rhs.degree());
        let mut Q = Polynomial::new(vec![T::zero(); self.terms.len()]);

        let mut R = self.clone();
        let mut D = rhs.clone();

        while R.degree() >= D.degree() && !R.is_zero() {
            let lead_r = if let Some(term) = R.lead_term() {
                term
            } else {
                panic!("No leading term.")
            };
            let lead_d = if let Some(term) = D.lead_term() {
                term
            } else {
                panic!("No leading term.")
            };
            let t = lead_r.clone() / lead_d.clone();
            Q = Q + t.clone();
            let subtrahend = D.clone() * t.clone();
            R = R - subtrahend;
        }

        (Q, R)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_univariate() {

        let mut cases: HashMap<(Polynomial<i32>, Polynomial<i32>), Polynomial<i32>> = HashMap::new();

        cases.insert(
            (Polynomial::new(vec![1, 2, 3]), Polynomial::new(vec![1, 2, 3])),
            Polynomial::new(vec![2, 4, 6])
        );
        cases.insert(
            (Polynomial::new(vec![1, 0, 1]), Polynomial::new(vec![0, 1, 0])),
            Polynomial::new(vec![1, 1, 1]),
        );
        cases.insert(
            (Polynomial::new(vec![-1, -2, -3]), Polynomial::new(vec![-1, -2, -3])),
            Polynomial::new(vec![-2, -4, -6]),
        );
        cases.insert(
            (Polynomial::new(vec![0, 0, 0]), Polynomial::new(vec![0, 0, 0])),
            Polynomial::new(vec![0, 0, 0]),
        );
        cases.insert(
            (Polynomial::new(vec![1, 1]), Polynomial::new(vec![0, 0, 1, 1])),
            Polynomial::new(vec![1, 1, 1, 1]),
        );

        for (addends, sum) in &cases {
            let (x, y) = addends;
            assert!(x.clone() + y.clone() == *sum);
        }

    }

    #[test]
    fn sub_univariate() {

        let mut cases: HashMap<(Polynomial<i32>, Polynomial<i32>), Polynomial<i32>> = HashMap::new();

        cases.insert(
            (Polynomial::new(vec![1, 2, 3]), Polynomial::new(vec![1, 2, 3])),
            Polynomial::new(vec![0, 0, 0])
        );
        cases.insert(
            (Polynomial::new(vec![1, 0, 1]), Polynomial::new(vec![0, 1, 0])),
            Polynomial::new(vec![1, -1, 1]),
        );
        cases.insert(
            (Polynomial::new(vec![-5, -2, -3]), Polynomial::new(vec![2, 3, 6])),
            Polynomial::new(vec![-7, -5, -9]),
        );
        cases.insert(
            (Polynomial::new(vec![0, 0, 0]), Polynomial::new(vec![0, 0, 0])),
            Polynomial::new(vec![0, 0, 0]),
        );
        cases.insert(
            (Polynomial::new(vec![1, 1]), Polynomial::new(vec![0, 0, 1, 1])),
            Polynomial::new(vec![1, 1, -1, -1]),
        );

        for (subtrahends, diff) in &cases {
            let (x, y) = subtrahends;
            assert!(x.clone() - y.clone() == *diff);
        }

    }

    #[test]
    fn mul_univariate() {

        let mut cases: HashMap<(Polynomial<i32>, Polynomial<i32>), Polynomial<i32>> = HashMap::new();

        cases.insert(
            (Polynomial::new(vec![1, 2, 3]), Polynomial::new(vec![1, 2, 3])),
            Polynomial::new(vec![1, 4, 10, 12, 9])
        );
        cases.insert(
            (Polynomial::new(vec![1, 0, -5]), Polynomial::new(vec![0, 10, -4])),
            Polynomial::new(vec![0, 10, -4, -50, 20])
        );
        cases.insert(
            (Polynomial::new(vec![0, 0, -1]), Polynomial::new(vec![1, -5, 0])),
            Polynomial::new(vec![0, 0, -1, 5, 0])
        );
        cases.insert(
            (Polynomial::new(vec![1, 1, 1]), Polynomial::new(vec![0, 0, 0])),
            Polynomial::new(vec![0, 0, 0, 0, 0])
        );
        cases.insert(
            (Polynomial::new(vec![0, 0, 1, 0, 0, -4]), Polynomial::new(vec![3, 1])),
            Polynomial::new(vec![0, 0, 3, 1, 0, -12, -4])
        );
        cases.insert(
            (Polynomial::new(vec![3, 1]), Polynomial::new(vec![0, 0, 1, 0, 0, -4])),
            Polynomial::new(vec![0, 0, 3, 1, 0, -12, -4])
        );


        for (factors, product) in &cases {
            let (x, y) = factors;
            assert!(x.clone() * y.clone() == *product);
        }

    }
    
}
