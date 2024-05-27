use num::Num;
use num::Zero;
use std::fmt;
use std::ops;

#[derive(Clone, Debug)]
pub struct Term<T> {
    pub coefficient: T,
    pub degree: usize,
}

#[derive(Clone, Debug)]
pub struct Polynomial<T> {
    pub terms: Vec<Term<T>>,
}

impl<T> Term<T>
where
    T: Num + Clone + Zero,
{
    fn new(coefficient: T, degree: usize) -> Self {
        Term {
            coefficient,
            degree,
        }
    }
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

impl<T: fmt::Display> fmt::Display for Term<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x^{}", self.coefficient, self.degree)
    }
}

impl<T> ops::Div<Term<T>> for Term<T>
where
    T: Num + Clone,
{
    type Output = Term<T>;

    fn div(self, rhs: Term<T>) -> Term<T> {
        Term::new(self.coefficient / rhs.coefficient, self.degree - rhs.degree)
    }
}

impl<T> ops::Sub<Term<T>> for Term<T>
where
    T: Num + Clone,
{
    type Output = Term<T>;

    fn sub(self, rhs: Term<T>) -> Term<T> {
        Term::new(self.coefficient - rhs.coefficient, self.degree)
    }
}

impl<T> ops::Mul<Term<T>> for Term<T>
where
    T: Num + Clone,
{
    type Output = Term<T>;

    fn mul(self, rhs: Term<T>) -> Term<T> {
        Term::new(self.coefficient * rhs.coefficient, self.degree + rhs.degree)
    }
}
impl<T> ops::Add<Term<T>> for Term<T>
where
    T: Num + Clone,
{
    type Output = Term<T>;

    fn add(self, rhs: Term<T>) -> Term<T> {
        Term::new(self.coefficient + rhs.coefficient, self.degree)
    }
}

impl<T: fmt::Display + Zero > fmt::Display for Polynomial<T> {
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

        let mut N = self.clone();
        let D = rhs.clone();

        while N.degree() >= D.degree() {
            let mut d_terms = D.clone().terms;
            d_terms.rotate_right(N.degree() - D.degree());
            let mut d = Polynomial::new(d_terms.into_iter().map(|t| t.coefficient).collect());
            Q.terms[N.degree() - D.degree()] =
                N.terms[N.degree()].clone() / d.terms[D.degree()].clone();
            let mut factor = Polynomial::new(vec![T::zero(); rhs.terms.len()]);
            factor.terms[N.degree() - D.degree()] = Q.terms[N.degree() - D.degree()].clone();
            d = d.clone() * factor.clone();
            N = N.clone() - d.clone();
            println!("{}", N.clone());
            break;
        }

        (Q, N.clone())
    }
}
