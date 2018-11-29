use std::{fmt, ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign}};

/// Unsigned normalized float in the range from 0 <= x <= 1 represented as an unsigend 32 Bit
/// integer.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Unorm(u32);

impl Unorm {
    /// Creates an Unorm from an u32. The returned type is binary identical to the passed u32, but
    /// the new value is interpreted to be `repr / (2^32 - 1)`.
    pub fn from_u32_repr(repr: u32) -> Unorm {
        Unorm(repr)
    }

    /// Creates an Unorm interpreted to be `1 / denom`.
    pub fn from_denominator(denom: u32) -> Unorm {
        Unorm(std::u32::MAX / denom)
    }

    /// Creates an Unorm from nominator and denominator.
    pub fn from_rational(nominator: u32, denom: u32) -> Unorm {
        assert!(nominator <= denom);
        Unorm(((nominator as u64 * std::u32::MAX as u64) / denom as u64) as u32)
    }
}

impl From<Unorm> for f64 {
    fn from(source: Unorm) -> f64 {
        source.0 as f64 / std::u32::MAX as f64
    }
}

impl From<Unorm> for f32 {
    fn from(source: Unorm) -> f32 {
        source.0 as f32 / std::u32::MAX as f32
    }
}

impl fmt::Display for Unorm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out : f64 = (*self).into();
        write!(f, "{}", out)
    }
}

/// `1 / (2^32 - 1)`. Smallest number different from zero representable by Unorm.
pub const EPSILON: Unorm = Unorm(1);
/// `1`. Largest number representable by Unorm.
pub const ONE: Unorm = Unorm(std::u32::MAX);
/// `0`. Smallest number representable by Unorm.
pub const ZERO: Unorm = Unorm(0);
/// `1`. Largest number representable by Unorm.
pub const MAX: Unorm = ONE;
/// `0`. Smallest number representable by Unorm.
pub const MIN: Unorm = ZERO;

impl Add for Unorm {
    type Output = Unorm;

    fn add(self, other: Unorm) -> Unorm {
        Unorm(self.0 + other.0)
    }
}

impl<'a> Add<&'a Unorm> for Unorm {
    type Output = Unorm;

    fn add(self, other: &'a Unorm) -> Unorm {
        Unorm(self.0 + other.0)
    }
}

impl Sub for Unorm {
    type Output = Unorm;

    fn sub(self, other: Unorm) -> Unorm {
        Unorm(self.0 - other.0)
    }
}

impl<'a> Sub<&'a Unorm> for Unorm {
    type Output = Unorm;

    fn sub(self, other: &'a Unorm) -> Unorm {
        Unorm(self.0 - other.0)
    }
}

impl AddAssign for Unorm {
    fn add_assign(&mut self, other: Unorm) {
        self.0 += other.0;
    }
}

impl<'a> AddAssign<&'a Unorm> for Unorm {
    fn add_assign(&mut self, other: &'a Unorm) {
        self.0 += other.0;
    }
}

impl SubAssign for Unorm {
    fn sub_assign(&mut self, other: Unorm) {
        self.0 -= other.0;
    }
}

impl<'a> SubAssign<&'a Unorm> for Unorm {
    fn sub_assign(&mut self, other: &'a Unorm) {
        self.0 -= other.0;
    }
}

impl Mul for Unorm {
    type Output = Unorm;

    fn mul(self, other: Unorm) -> Unorm {
        Unorm(((self.0 as u64 * other.0 as u64) / std::u32::MAX as u64) as u32)
    }
}

impl<'a> Mul<&'a Unorm> for Unorm {
    type Output = Unorm;

    fn mul(self, other: &'a Unorm) -> Unorm {
        self * (*other)
    }
}

impl MulAssign for Unorm {
    fn mul_assign(&mut self, other: Unorm) {
        *self = (*self) * other;
    }
}

impl<'a> MulAssign<&'a Unorm> for Unorm {
    fn mul_assign(&mut self, other: &'a Unorm) {
        *self = (*self) * (*other);
    }
}

#[cfg(test)]
mod tests {

    use super::{Unorm, ONE};

    #[test]
    fn construction() {
        assert_eq!(
            Unorm::from_denominator(2),
            Unorm::from_u32_repr((1 << 31) - 1)
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            Unorm::from_denominator(4),
            Unorm::from_denominator(2) * Unorm::from_denominator(2)
        );

        assert_eq!(
            Unorm::from_rational(2, 9),
            Unorm::from_rational(1, 3) * Unorm::from_rational(2, 3)
        );

        assert_eq!(ONE, ONE * ONE);
    }
}
