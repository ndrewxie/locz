use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fixed<const SCALE: usize>(i32);
impl<const SCALE: usize> Fixed<SCALE> {
    pub fn new(val: i32) -> Self {
        Self(val * (1 << SCALE))
    }
    pub fn as_i32(self) -> i32 {
        self.0 / (1 << SCALE)
    }
    pub fn as_f32(self) -> f32 {
        (self.0 as f32) / ((1 << SCALE) as f32)
    }
    pub fn trunc(self) -> Self {
        Self((self.0 / (1 << SCALE)) * (1 << SCALE))
    }
    pub fn frac(self) -> Self {
        if self.is_neg() {
            return self.trunc() - self;
        }
        self - self.trunc()
    }
    pub fn floor(self) -> Self {
        if self.is_neg() {
            return (self - 1).trunc();
        }
        self.trunc()
    }
    pub fn ceil(self) -> Self {
        if self.trunc() == self {
            return self;
        }
        (self + 1).floor()
    }
    /*
    pub fn floor(self) -> Self {
        Self(self.0 & !((1 << SCALE)-1))
    }
    pub fn ceil(self) -> Self {
        if self.0 & ((1 << SCALE)-1) == 0 {
            return self;
        }
        (self + 1).floor()
    }
    */
    pub fn div_rem_euclid(self, other: Self) -> (Self, Self) {
        (
            Self(self.0.div_euclid(other.0) * (1 << SCALE)),
            Self(self.0.rem_euclid(other.0)),
        )
    }
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
    pub fn is_neg(self) -> bool {
        self.0 < 0
    }
    pub fn is_pos(self) -> bool {
        self.0 > 0
    }
}
impl<const SCALE: usize> Add<Fixed<SCALE>> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}
impl<const SCALE: usize> Add<i32> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn add(self, other: i32) -> Self {
        Self(self.0 + other * (1 << SCALE))
    }
}
impl<const SCALE: usize> Sub<Fixed<SCALE>> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}
impl<const SCALE: usize> Sub<i32> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn sub(self, other: i32) -> Self {
        Self(self.0 - other * (1 << SCALE))
    }
}
impl<const SCALE: usize> Mul<Fixed<SCALE>> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn mul(self, other: Self) -> Self {
        Self(((self.0 as i64 * other.0 as i64) / (1 << SCALE)) as i32)
    }
}
impl<const SCALE: usize> Mul<i32> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn mul(self, other: i32) -> Self {
        Self(self.0 * other)
    }
}
impl<const SCALE: usize> Div<Fixed<SCALE>> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn div(self, other: Self) -> Self {
        Self(((self.0 as i64 * (1 << SCALE)) / other.0 as i64) as i32)
    }
}
impl<const SCALE: usize> Div<i32> for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn div(self, other: i32) -> Self {
        Self(self.0 / other)
    }
}
impl<const SCALE: usize> Neg for Fixed<SCALE> {
    type Output = Fixed<SCALE>;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl<const SCALE: usize> From<i32> for Fixed<SCALE> {
    fn from(input: i32) -> Self {
        Self(input * (1 << SCALE))
    }
}
impl<const SCALE: usize> From<f32> for Fixed<SCALE> {
    fn from(input: f32) -> Self {
        Self((input * (1 << SCALE) as f32) as i32)
    }
}
impl<const SCALE: usize> PartialOrd for Fixed<SCALE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<const SCALE: usize> Ord for Fixed<SCALE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<const SCALE: usize> std::fmt::Display for Fixed<SCALE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fixed({},<{}>)", self.as_f32(), self.0)
    }
}
impl<const SCALE: usize> std::fmt::Debug for Fixed<SCALE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fixed({},<{}>)", self.as_f32(), self.0)
    }
}
