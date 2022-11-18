use std::fmt::{self, Formatter, Display};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    x1: f64,
    x2: f64,
    polar: bool
}

#[allow(dead_code)]
impl Complex {
    pub const ZERO: Complex = Complex{x1:0.0, x2:0.0, polar:true};
    pub const ONE: Complex = Complex{x1:1.0, x2:0.0, polar:true};
    pub const I: Complex = Complex{x1:1.0, x2:0.5, polar:true};
    pub const MINUS_I: Complex = Complex{x1:1.0, x2:1.5, polar:true};
    pub const MINUS_ONE: Complex = Complex{x1:1.0, x2:1.0, polar:true};
    const PI: f64 = std::f64::consts::PI;

    pub fn abs(&self) -> f64 {
        if self.polar {
            self.x1
        } else {
            (self.x1 * self.x1 + self.x2 * self.x2).sqrt()
        }
    }

    pub fn abs_sq(&self) -> f64 {
        if self.polar {
            self.x1 * self.x1
        } else {
            self.x1 * self.x1 + self.x2 * self.x2
        }
    }

    pub fn arg(&self) -> f64 {
        if self.polar {
            self.x2 * Complex::PI
        } else {
            self.x2.atan2(self.x1)
        }
    }

    pub fn arg_dev_pi(&self) -> f64 {
        if self.polar {
            self.x2
        } else {
            self.x2.atan2(self.x1) / Complex::PI
        }
    }

    pub fn re(&self) -> f64 {
        if self.polar {
            self.x1 * (self.x2 * Complex::PI).cos()
        } else {
            self.x1
        }
    }

    pub fn im(&self) -> f64 {
        if self.polar {
            self.x1 * (self.x2 * Complex::PI).sin()
        } else {
            self.x2
        }
    }

    pub fn cc(&self) -> Complex {
        Complex {
            x1: self.x1,
            x2: -self.x2,
            polar: self.polar
        }
    }

    pub fn exp(&self) -> Complex {
        Complex::new_polar(self.re().exp(), self.im() / Complex::PI)
    }

    pub fn log(&self) -> Complex {
        Complex::new_cart(self.re().ln(), self.arg())
    }

    pub fn is_zero(&self) -> bool {
        if self.polar {
            self.x1 == 0.0
        } else {
            self.x1 == 0.0 && self.x2 == 0.0
        }
    }

    pub fn convert_to_polar(&self) -> Complex {
        Complex {
            x1: self.abs(),
            x2: self.arg_dev_pi(),
            polar: true
        }
    }

    pub fn convert_to_cart(&self) -> Complex {
        Complex {
            x1: self.re(),
            x2: self.im(),
            polar: true
        }
    }

    pub fn prod_i(&self) -> Complex {
        if self.polar {
            Complex {
                x1: self.x1,
                x2: self.x2 + 0.5,
                polar: true
            }
        } else {
            Complex {
                x1: -self.x2,
                x2: self.x1,
                polar: false
            }
        }
    }

    pub fn toggle_sign(&self) -> Complex {
        if self.polar {
            Complex {
                x1: self.x1,
                x2: self.x2 + 1.0,
                polar: true
            }
        } else {
            Complex {
                x1: -self.x1,
                x2: -self.x2,
                polar: false
            }
        }
    }

    pub fn new_cart(r: f64, i: f64) -> Complex {
        Complex {
            x1: r,
            x2: i,
            polar: false
        }
    }

    pub fn new_polar(r: f64, q: f64) -> Complex {
        Complex{
            x1: r,
            x2: q,
            polar: true
        }
    }

    pub fn new_real(r: f64) -> Complex {
        Complex{
            x1: r,
            x2: 0.0,
            polar: true
        }
    }

    pub fn new_imaginal(i: f64) -> Complex {
        Complex{
            x1: i,
            x2: 1.0,
            polar: true
        }
    }

    pub fn exp_i_pi_theta(theta: f64) -> Complex {
        Complex {
            x1: 1.0,
            x2: theta,
            polar: true
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.polar {
            write!(f, "{} x exp({}πj)", self.x1, self.x2)
        } else {
            write!(f, "{}+{}j", self.x1, self.x2)
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, _rhs: Complex) -> Complex {
        Complex{
            x1: self.re() + _rhs.re(),
            x2: self.im() + _rhs.im(),
            polar: false
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x1: self.re() + other.re(),
            x2: self.im() + other.im(),
            polar: false
        };
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, _rhs: Complex) -> Complex {
        Complex{
            x1: self.re() - _rhs.re(),
            x2: self.im() - _rhs.im(),
            polar: false
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        *self = Complex{
            x1: self.re() - other.re(),
            x2: self.im() - other.im(),
            polar: false
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, _rhs: Complex) -> Complex {
        if self.polar && _rhs.polar {
            Complex {
                x1: self.x1 * _rhs.x1,
                x2: self.x2 + _rhs.x2,
                polar: true
            }
        } else {
            Complex {
                x1: self.re() * _rhs.re() - self.im() * _rhs.im(),
                x2: self.re() * _rhs.im() + self.im() * _rhs.re(),
                polar: false
            }
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, _rhs: f64) -> Complex {
        if self.polar {
            Complex{
                x1: self.x1 * _rhs,
                x2: self.x2,
                polar: true
            }
        } else {
            Complex{
                x1: self.x1 * _rhs,
                x2: self.x2 * _rhs,
                polar: false
            }
        }
    }
}

impl Mul<Complex> for f64 {
    type Output = Complex;

    fn mul(self, _rhs: Complex) -> Complex {
        if _rhs.polar {
            Complex{
                x1: _rhs.x1 * self,
                x2: _rhs.x2,
                polar: true
            }
        } else {
            Complex{
                x1: self * _rhs.x1,
                x2: self * _rhs.x2,
                polar: false
            }
        }
    }
}

impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, other: Complex) {
        if self.polar && other.polar {
            *self = Self {
                x1: self.x1 * other.x1,
                x2: self.x2 + other.x2,
                polar: true
            };
        } else {
            *self = Self {
                x1: self.re() * other.re() - self.im() * other.im(),
                x2: self.re() * other.im() + self.im() * other.re(),
                polar: false
            }
        }
    }
}

impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, other: f64) {
        if self.polar {
            *self = Self {
                x1: self.x1 * other,
                x2: self.x2,
                polar: true
            };
        } else {
            *self = Self {
                x1: self.x1 * other,
                x2: self.x2 * other,
                polar: false
            };
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, _rhs: Complex) -> Complex {
        if _rhs.is_zero() {
            panic!("attempt to divide by zero");
        } else if self.polar && _rhs.polar {
            Complex {
                x1: self.x1 / _rhs.x1,
                x2: self.x2 - _rhs.x2,
                polar: true
            }
        } else {
            Complex {
                x1: (self.re() * _rhs.re() + self.im() * _rhs.im()) / _rhs.abs_sq(),
                x2: (-self.re() * _rhs.im() + self.im() * _rhs.re()) / _rhs.abs_sq(),
                polar: false
            }
        }
    }
}

impl Div<f64> for Complex {
    type Output = Complex;

    fn div(self, _rhs: f64) -> Complex {
        if _rhs == 0.0 {
            panic!("attempt to divide by zero")
        } else if self.polar {
            Complex {
                x1: self.x1 / _rhs,
                x2: self.x2,
                polar: true
            }
        } else {
            Complex {
                x1: self.x1 / _rhs,
                x2: self.x2 / _rhs,
                polar: false
            }
        }
    }
}

impl Div<Complex> for f64 {
    type Output = Complex;

    fn div(self, _rhs: Complex) -> Complex {
        if _rhs.is_zero() {
            panic!("attempt to divide by zero")
        } else if _rhs.polar {
            Complex {
                x1: self / _rhs.x1,
                x2: -_rhs.x2,
                polar: true
            }
        } else {
            Complex {
                x1: self * _rhs.x1 / _rhs.abs_sq(),
                x2: -self * _rhs.x2 / _rhs.abs_sq(),
                polar: false
            }
        }
    }
}

impl DivAssign<Complex> for Complex {
    fn div_assign(&mut self, other: Self) {
        if other.is_zero() {
            panic!("attempt to divide by zero");
        } else if self.polar && other.polar {
            *self = Self {
                x1: self.x1 / other.x1,
                x2: self.x2 - other.x2,
                polar: true
            };
        } else {
            *self = Self {
                x1: (self.re() * other.re() + self.im() * other.im()) / other.abs_sq(),
                x2: (-self.re() * other.im() + self.im() * other.re()) / other.abs_sq(),
                polar: false
            };
        }
    }
}

impl DivAssign<f64> for Complex {
    fn div_assign(&mut self, other: f64) {
        if other == 0.0 {
            panic!("attempt to divide by zero")
        } else if self.polar {
            *self = Self {
                x1: self.x1 / other,
                x2: self.x2,
                polar: true
            };
        } else {
            *self = Self {
                x1: self.re() / other,
                x2: self.im() / other,
                polar: false
            };
        }
    }
}