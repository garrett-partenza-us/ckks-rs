use crate::poly::term::Term;
use num::Num;
use num::Zero;
use std::fmt;
use std::ops;

#[derive(Clone, Debug)]
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
        let terms: Vec<Term<T>> = self
            .terms
            .iter()
            .zip(rhs.terms.iter())
            .map(|(l, r)| (*l).clone() + (*r).clone())
            .collect();
        Polynomial { terms: terms }
    }
}

impl<T> ops::Mul<Term<T>> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;
    fn mul(mut self, rhs: Term<T>) -> Polynomial<T> {
        if rhs.degree > self.degree() {
            for d in (self.degree())..rhs.degree {
                self.terms.push(Term::new(T::zero(), d));
            }
        }
        self.terms[rhs.degree] = self.terms[rhs.degree].clone() * rhs.clone();
        Polynomial { terms: self.terms }
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
        let terms: Vec<Term<T>> = self
            .terms
            .iter()
            .zip(rhs.terms.iter())
            .map(|(l, r)| l.clone() - r.clone())
            .collect();
        Polynomial { terms: terms }
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

        while R.degree() >= D.degree() && true {
            // replace true with r != zero
            let t: Term<T> = R.terms.last().expect("Vector cannot be empty").clone()
                / D.terms.last().expect("Vector cannot be empty").clone();
            Q = Q + t.clone();
            R = R - (D.clone() * t.clone());
        }

        (Q, R)
    }
}
