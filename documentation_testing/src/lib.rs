/// Function to add two integers together
///
/// ```
/// assert_eq!(documentation_testing::add(10, 20), 30);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Divide a bigger integer by a smaller non-zero integer
///
/// # Examples
///
/// ```
/// assert_eq!(documentation_testing::divide_by_non_zero(10, 2), 5);
/// ```
///
/// # Panics
///
/// ```rust,should_panic
/// assert_eq!(documentation_testing::divide_by_non_zero(10, 0), 10);
/// ```
pub fn divide_by_non_zero(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("tried to divide by zero");
    } else if x < y {
        panic!("division result is zero");
    }

    x / y
}

/// Divide an integer by a non-zero integer - error in case of zero value for divisor.
///
/// # Examples
///
///```
/// # fn try_main() -> Result<(), String> {
/// let res = documentation_testing::try_div(10, 2)?;
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # try_main().unwrap();
/// # }
/// ```
pub fn try_div(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(x / y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add() {
        assert_eq!(add(1, 3), 4);
    }
}
