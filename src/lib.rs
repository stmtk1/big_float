use std::cmp::max;

pub struct BigFloat{
    base: i64,
    exp: i64,
}

impl BigFloat {
    pub fn new(base: i64, exp: i64) -> BigFloat {
        BigFloat { base, exp, }
    }
    
    pub fn add(a: BigFloat, b: BigFloat) -> BigFloat {
        let new_exp = max(a.exp, b.exp);
        let base = (a.base << (new_exp - a.exp)) + (b.base << (new_exp - b.exp));
        BigFloat { 
            base, 
            exp: new_exp 
        }
    }
    
    fn add_inverse(a: BigFloat) -> BigFloat {
        BigFloat { 
            base: -1 * a.base,
            exp: a.exp 
        }
    }
    
    pub fn minus(a: BigFloat, b: BigFloat) -> BigFloat {
        BigFloat::add(a, BigFloat::add_inverse(b))
    }
    
    pub fn multiply(a: BigFloat, b: BigFloat) -> BigFloat {
        BigFloat {
            base: a.base * b.base,
            exp: a.exp + b.exp,
        }
    }
    
    fn mult_inverse(a: BigFloat) -> BigFloat {
        BigFloat { 
            base: a.base,
            exp: -1 * a.exp 
        }
    }
    
    pub fn divide(a: BigFloat, b: BigFloat) -> BigFloat {
        BigFloat::multiply(a, BigFloat::mult_inverse(b))
    }
}

#[cfg(test)]
mod tests {
    use super::BigFloat;
    #[test]
    fn new_test() {
        let a = BigFloat::new(3, 2);
        assert_eq!(a.base, 3);
        assert_eq!(a.exp, 2);
    }
    
    #[test]
    fn add_test() {
        let a = BigFloat::new(3, 2);
        let b = BigFloat::new(5, 2);
        let ret = BigFloat::add(a, b);
        assert_eq!(ret.base, 8);
        assert_eq!(ret.exp, 2);
    }
    
    #[test]
    fn add_inverse_test(){
        let a = BigFloat::new(3, 2);
        let ret = BigFloat::add_inverse(a);
        assert_eq!(ret.base, -3);
        assert_eq!(ret.exp, 2);
    }
    
    #[test]
    fn minus_test(){
        let a = BigFloat::new(5, 2);
        let b = BigFloat::new(1, 2);
        let ret = BigFloat::minus(a, b);
        assert_eq!(ret.base, 4);
        assert_eq!(ret.exp, 2);
    }
    
    #[test]
    fn multiply_test(){
        let a = BigFloat::new(3, 7);
        let b = BigFloat::new(5, 11);
        let ret = BigFloat::multiply(a, b);
        assert_eq!(ret.base, 15);
        assert_eq!(ret.exp, 18);
    }
    
    #[test]
    fn mult_inverse_test(){
        let a = BigFloat::new(3, 7);
        let ret = BigFloat::mult_inverse(a);
        assert_eq!(ret.base, 3);
        assert_eq!(ret.exp, -7);
    }
    
    #[test]
    fn divide_test() {
        let a = BigFloat::new(6, 5);
        let b = BigFloat::new(2, 7);
        let ret = BigFloat::divide(a, b);
        assert_eq!(ret.base, 12);
        assert_eq!(ret.exp, -2);
    }
}
