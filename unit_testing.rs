pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn bad_add(x: i32, y: i32) -> i32 {
    x - y
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative numbers do not have  square roots.".to_string())
    }
}

pub fn divide_by_non_zero(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("divide by zero!");
    } else if x < y {
        panic!("divide result is zero");
    }

    x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[ignore]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_divide_by_non_zero_valid() {
        assert_eq!(divide_by_non_zero(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_non_zero_invalid() {
        assert_eq!(divide_by_non_zero(10, 0), 10);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_non_zero_invalid1() {
        assert_eq!(divide_by_non_zero(10, 200), 10);
    }
}
