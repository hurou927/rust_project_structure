use crate::sub3::mul;

pub fn mul_mul(l: usize, r:usize) -> usize {
    mul(l, r) * mul(l, r)
}
