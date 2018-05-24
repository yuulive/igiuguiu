// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter::Sum;

/// Used for sum Iterator<T>
/// 
/// # Arguments
/// 
/// * `it`: Iterator T
pub fn sum<T: Sum>(it: impl Iterator<Item=T>) -> T {
    it.sum()
}

/// Shorthand macro of sum
/// 
/// Syntax:
/// 1. sum!(0,5) // equals sum(0..=5)
/// 2. sum!(0,1,2,3,4,5)
#[macro_export]
macro_rules! sum {
    ($i:expr;$j:expr) => {{
        sum($i..=$j)
    }};
    ($i:expr,$($j:expr),*) => {
        $i + sum!($($j),*)
    };
    ($i:expr) => {
        $i
    };
}
