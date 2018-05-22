// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// This macro is used to provide shortcut of function composition.
/// The order is last-in-first-invoke.
/// 
/// # Arguments
/// 
/// * `args` - function or closure separated by comma(,)

#[macro_export]
macro_rules! pipe  {
    ($($f:expr),*) => {
        |v| {pipe!(@NEXT v, $($f),*) }
    };

    (@NEXT $v:expr,$f:expr,$($rest:expr),*) => {
        pipe!(@NEXT $f($v), $($rest),*)
    };

    (@NEXT $v:expr,$last:expr) => {
        $last($v)
    };
}