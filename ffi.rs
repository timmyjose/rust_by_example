use std::fmt;

#[link(name = "m")]
extern "C" {
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

fn main() {
    let z = Complex { re: -1.0, im: 0.0 };

    let z_sqrt = unsafe { csqrtf(z) };
    println!("the square root of {:?} is {:?}", z, z_sqrt);

    println!("cos({:?}) = {:?}", z, cos(z));
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{}-{}", self.re, self.im)
        } else {
            write!(f, "{}+{}", self.re, self.im)
        }
    }
}