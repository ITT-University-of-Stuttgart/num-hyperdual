//! Generalized, recursive, scalar and vector (hyper) dual numbers for the automatic and exact calculation of (partial) derivatives.
//!
//! ## Example
//! This example defines a generic function that can be called using any (hyper) dual number and automatically calculates derivatives.
//! ```
//! use num_hyperdual::*;
//!
//! fn f<D: DualNum<f64>>(x: D, y: D) -> D {
//!     x.powi(3) * y.powi(2)
//! }
//!
//! fn main() {
//!     let (x, y) = (5.0, 4.0);
//!
//!     // Calculate a simple derivative
//!     let x_dual = Dual64::from(x).derive();
//!     let y_dual = Dual64::from(y);
//!     println!("{}", f(x_dual, y_dual));                      // 2000 + 1200ε
//!
//!     // Calculate a gradient
//!     let x_dual2 = DualN64::<2>::from(x).derive(0);
//!     let y_dual2 = DualN64::<2>::from(y).derive(1);
//!     println!("{}", f(x_dual2, y_dual2).eps);                // [1200, 1000]
//!
//!     // Calculate a Hessian
//!     let x_hyperdual2 = HyperDualN64::<2>::from(x).derive(0);
//!     let y_hyperdual2 = HyperDualN64::<2>::from(y).derive(1);
//!     println!("{}", f(x_hyperdual2, y_hyperdual2).hessian);  // [[480, 600], [600, 250]]
//!
//!     // for x=cos(t) and y=sin(t) calculate the third derivative w.r.t. t
//!     let t = HD3_64::from(1.0).derive();
//!     println!("{}", f(t.cos(), t.sin()).v3);                 // 7.358639755305733
//! }
//! ```

use num_traits::{Float, FromPrimitive, Inv, NumAssignOps, NumOps, Signed};
use std::fmt;
use std::iter::{Product, Sum};

#[macro_use]
mod macros;
#[macro_use]
mod derivatives;

mod dual;
mod dual_n;
mod hd2;
mod hd3;
mod hyperdual;
mod hyperdual_n;
mod static_mat;
pub use dual::{Dual, Dual32, Dual64};
pub use dual_n::{DualN, DualN32, DualN64};
pub use hd2::{HD2Dual32, HD2Dual64, HD2DualN32, HD2DualN64, HD2, HD2_32, HD2_64};
pub use hd3::{HD3Dual32, HD3Dual64, HD3DualN32, HD3DualN64, HD3, HD3_32, HD3_64};
pub use hyperdual::{
    HyperDual, HyperDual32, HyperDual64, HyperDualDual32, HyperDualDual64, HyperDualDualN32,
    HyperDualDualN64,
};
pub use hyperdual_n::{
    HyperDualN, HyperDualN32, HyperDualN64, HyperDualNDual32, HyperDualNDual64, HyperDualNDualN32,
    HyperDualNDualN64,
};
pub use static_mat::{StaticMat, StaticVec};

#[cfg(feature = "linalg")]
mod linalg;
#[cfg(feature = "linalg")]
pub use linalg::*;

/// A generalized (hyper) dual number.
pub trait DualNum<F>:
    NumOps
    + Signed
    + NumOps<F>
    + NumAssignOps
    + NumAssignOps<F>
    + Clone
    + Copy
    + Inv<Output = Self>
    + Sum
    + Product
    + FromPrimitive
    + From<F>
    + fmt::Display
    + Sync
    + Send
    + 'static
{
    /// Highest derivative that can be calculated with this struct
    const NDERIV: usize;

    /// Multiply the number with the scalar f inplace.
    fn scale(&mut self, f: F);

    /// Real part (0th derivative) of the number
    fn re(&self) -> F;

    /// Reciprocal (inverse) of a number `1/x`.
    fn recip(&self) -> Self;

    /// Power with integer exponent `x^n`
    fn powi(&self, n: i32) -> Self;

    /// Power with real exponent `x^n`
    fn powf(&self, n: F) -> Self;

    /// Square root
    fn sqrt(&self) -> Self;
    /// Cubic root
    fn cbrt(&self) -> Self;

    /// Exponential `e^x`
    fn exp(&self) -> Self;

    /// Exponential with base 2 `2^x`
    fn exp2(&self) -> Self;

    /// Exponential minus 1 `e^x-1`
    fn exp_m1(&self) -> Self;

    /// Natural logarithm
    fn ln(&self) -> Self;

    /// Logarithm with arbitrary base
    fn log(&self, base: F) -> Self;

    /// Logarithm with base 2
    fn log2(&self) -> Self;

    /// Logarithm with base 10
    fn log10(&self) -> Self;

    /// Logarithm on x plus one `ln(1+x)`
    fn ln_1p(&self) -> Self;

    /// Sine
    fn sin(&self) -> Self;

    /// Cosine
    fn cos(&self) -> Self;

    /// Tangent
    fn tan(&self) -> Self;

    /// Calculate sine and cosine simultaneously
    fn sin_cos(&self) -> (Self, Self);

    /// Arcsine
    fn asin(&self) -> Self;

    /// Arccosine
    fn acos(&self) -> Self;

    /// Arctangent
    fn atan(&self) -> Self;

    /// Hyperbolic sine
    fn sinh(&self) -> Self;

    /// Hyperbolic cosine
    fn cosh(&self) -> Self;

    /// Hyperbolic tangent
    fn tanh(&self) -> Self;

    /// Area hyperbolic sine
    fn asinh(&self) -> Self;

    /// Area hyperbolic cosine
    fn acosh(&self) -> Self;

    /// Area hyperbolic tangent
    fn atanh(&self) -> Self;

    /// 0th order spherical bessel function of the first kind
    fn sph_j0(&self) -> Self;

    /// 1st order spherical bessel function of the first kind
    fn sph_j1(&self) -> Self;

    /// 2nd order spherical bessel function of the first kind
    fn sph_j2(&self) -> Self;

    /// Fused multiply-add
    #[inline]
    fn mul_add(&self, a: Self, b: Self) -> Self {
        self.clone() * a + b
    }

    /// Power with dual exponent `x^n`
    #[inline]
    fn powd(&self, exp: &Self) -> Self {
        (self.ln() * exp.clone()).exp()
    }
}

/// The underlying data type of individual derivatives. Usually f32 or f64.
pub trait DualNumFloat:
    Float + FromPrimitive + Signed + fmt::Display + Sync + Send + 'static
{
}
impl<T> DualNumFloat for T where
    T: Float + FromPrimitive + Signed + fmt::Display + Sync + Send + 'static
{
}

macro_rules! impl_dual_num_float {
    ($float:ty) => {
        impl DualNum<$float> for $float {
            const NDERIV: usize = 0;

            fn re(&self) -> $float {
                *self
            }

            fn scale(&mut self, f: $float) {
                *self *= f;
            }

            fn mul_add(&self, a: Self, b: Self) -> Self {
                <$float>::mul_add(*self, a, b)
            }
            fn recip(&self) -> Self {
                <$float>::recip(*self)
            }
            fn powi(&self, n: i32) -> Self {
                <$float>::powi(*self, n)
            }
            fn powf(&self, n: Self) -> Self {
                <$float>::powf(*self, n)
            }
            fn powd(&self, n: &Self) -> Self {
                <$float>::powf(*self, *n)
            }
            fn sqrt(&self) -> Self {
                <$float>::sqrt(*self)
            }
            fn exp(&self) -> Self {
                <$float>::exp(*self)
            }
            fn exp2(&self) -> Self {
                <$float>::exp2(*self)
            }
            fn ln(&self) -> Self {
                <$float>::ln(*self)
            }
            fn log(&self, base: Self) -> Self {
                <$float>::log(*self, base)
            }
            fn log2(&self) -> Self {
                <$float>::log2(*self)
            }
            fn log10(&self) -> Self {
                <$float>::log10(*self)
            }
            fn cbrt(&self) -> Self {
                <$float>::cbrt(*self)
            }
            fn sin(&self) -> Self {
                <$float>::sin(*self)
            }
            fn cos(&self) -> Self {
                <$float>::cos(*self)
            }
            fn tan(&self) -> Self {
                <$float>::tan(*self)
            }
            fn asin(&self) -> Self {
                <$float>::asin(*self)
            }
            fn acos(&self) -> Self {
                <$float>::acos(*self)
            }
            fn atan(&self) -> Self {
                <$float>::atan(*self)
            }
            fn sin_cos(&self) -> (Self, Self) {
                <$float>::sin_cos(*self)
            }
            fn exp_m1(&self) -> Self {
                <$float>::exp_m1(*self)
            }
            fn ln_1p(&self) -> Self {
                <$float>::ln_1p(*self)
            }
            fn sinh(&self) -> Self {
                <$float>::sinh(*self)
            }
            fn cosh(&self) -> Self {
                <$float>::cosh(*self)
            }
            fn tanh(&self) -> Self {
                <$float>::tanh(*self)
            }
            fn asinh(&self) -> Self {
                <$float>::asinh(*self)
            }
            fn acosh(&self) -> Self {
                <$float>::acosh(*self)
            }
            fn atanh(&self) -> Self {
                <$float>::atanh(*self)
            }
            fn sph_j0(&self) -> Self {
                if self.abs() < <$float>::EPSILON {
                    1.0 - self * self / 6.0
                } else {
                    self.sin() / self
                }
            }
            fn sph_j1(&self) -> Self {
                if self.abs() < <$float>::EPSILON {
                    self / 3.0
                } else {
                    let sc = self.sin_cos();
                    let rec = self.recip();
                    (sc.0 * rec - sc.1) * rec
                }
            }
            fn sph_j2(&self) -> Self {
                if self.abs() < <$float>::EPSILON {
                    self * self / 15.0
                } else {
                    let sc = self.sin_cos();
                    let s2 = self * self;
                    ((3.0 - s2) * sc.0 - 3.0 * self * sc.1) / (self * s2)
                }
            }
        }
    };
}

impl_dual_num_float!(f32);
impl_dual_num_float!(f64);
