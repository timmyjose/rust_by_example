fn main() {
    println!("{}", checked::op(1.0, 10.0));
}

mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    pub fn op(x: f64, y: f64) -> f64 {
        match op_(x, y) {
            Err(err) => panic!(match err {
                MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                MathError::NegativeSquareRoot => "square root of negative number",
                MathError::DivisionByZero => "attempting to divide by zero",
            }),
            Ok(res) => res,
        }
    }

    fn op_(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;

        sqrt(ln)
    }
}
