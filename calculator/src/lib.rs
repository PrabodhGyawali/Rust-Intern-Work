struct Calculator;

impl Calculator {
    fn add(a: i32, b:i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    fn multiply(a: i32, b:i32) -> i32 {
        a*b
    }
    fn divide(a: i32, b:i32) -> Option<f32> {
        if b == 0{
            None
        } else {
            Some(a as f32 / b as f32)
        }
    }

    // fn vector_add(v1: Vec<f32>, v2: Vec<f32>) -> Vec<f32> {
       
       
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Calculator::add(5, 5), 10);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Calculator::subtract(5, 3), 2);
    }
    #[test]
    fn test_multiply() {
        assert_eq!(Calculator::multiply(4, 3), 12);
    }

    #[test]
    fn test_divide() {
        assert_eq!(Calculator::divide(10, 2), Some(5.0));
        assert_eq!(Calculator::divide(10, 0), None);
    }
}

