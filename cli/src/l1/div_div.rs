use crate::l2::power::power;

use super::div::div;

pub fn div_div(l: usize, r:usize) -> usize {
    div(l, r) / div(l, r)
}

pub fn div_power(l: usize, r:usize) -> usize {
    div(l, r) * power(l, r)
}
