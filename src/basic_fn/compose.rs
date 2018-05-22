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
macro_rules! compose {
    ($f:expr,$($fs:expr),*) => {
        compose!(@NEXT [$f],$($fs),*)
    };
    (@NEXT [$($fs:expr),*],$f:expr,$($remain_fs:expr),*) => {
        compose!(@NEXT [$f,$($fs),*],$($remain_fs),*)
    };
    (@NEXT [$($fs:expr),*],$f:expr) => {
        pipe!($f,$($fs),*)
    };
}