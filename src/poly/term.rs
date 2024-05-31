use num::Num;
use num::Zero;
use std::fmt;
use std::ops;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Term<T> {
    pub coefficient: T,
    pub degree: usize,
}

impl<T> Term<T>
where
    T: Num + Clone + Zero,
{
    pub fn new(coefficient: T, degree: usize) -> Self {
        Term {
            coefficient,
            degree,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn add_terms() {

        let mut cases: HashMap<(Term<i32>, Term<i32>), Term<i32>> = HashMap::new();

        cases.insert((Term::new(0,0), Term::new(0,0)), Term::new(0,0));
        cases.insert((Term::new(1,1), Term::new(1,1)), Term::new(2,1));
        cases.insert((Term::new(2,2), Term::new(2,2)), Term::new(4,2));
        cases.insert((Term::new(5,3), Term::new(5,3)), Term::new(10,3));

        for (addends, sum) in &cases {
            let (x, y) = addends;
            assert!(*x + *y == *sum);
        }

    }
    
    #[test]
    fn sub_terms() {

        let mut cases: HashMap<(Term<i32>, Term<i32>), Term<i32>> = HashMap::new();

        cases.insert((Term::new(0,0), Term::new(0,0)), Term::new(0,0));
        cases.insert((Term::new(1,1), Term::new(1,1)), Term::new(0,1));
        cases.insert((Term::new(5,2), Term::new(3,2)), Term::new(2,2));
        cases.insert((Term::new(3,2), Term::new(5,2)), Term::new(-2,2));

        for (subtrahends, diff) in &cases {
            let (x, y) = subtrahends;
            assert!(*x - *y == *diff);
        }

    }
    
    #[test]
    fn mul_terms() {

        let mut cases: HashMap<(Term<i32>, Term<i32>), Term<i32>> = HashMap::new();

        cases.insert((Term::new(0,0), Term::new(0,0)), Term::new(0,0));
        cases.insert((Term::new(1,1), Term::new(1,1)), Term::new(1,2));
        cases.insert((Term::new(5,2), Term::new(3,2)), Term::new(15,4));
        cases.insert((Term::new(2,5), Term::new(3,3)), Term::new(6,8));

        for (factors, product) in &cases {
            let (x, y) = factors;
            assert!(*x * *y == *product);
        }

    }
    
    #[test]
    fn div_terms() {

        let mut cases: HashMap<(Term<i32>, Term<i32>), Term<i32>> = HashMap::new();

        cases.insert((Term::new(1,0), Term::new(1,0)), Term::new(1,0));
        cases.insert((Term::new(1,1), Term::new(1,1)), Term::new(1,0));
        cases.insert((Term::new(15,2), Term::new(3,2)), Term::new(5,0));
        cases.insert((Term::new(10,5), Term::new(2,3)), Term::new(5,2));

        for (terms, quotient) in &cases {
            let (x, y) = terms;
            assert!(*x / *y == *quotient);
        }

    }
}

