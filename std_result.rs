fn main() {
    println!("{}", op(1.0, 10.0));
}

fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(err) => panic!("{:?}", err),
        Ok(div_res) => match checked::ln(div_res) {
            Err(err) => panic!("{:?}", err),
            Ok(ln_res) => match checked::sqrt(ln_res) {
                Err(err) => panic!("{:?}", err),
                Ok(val) => val,
            },
        },
    }
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
}
