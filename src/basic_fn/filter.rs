// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Used for filter Iterator<T>
/// 
/// # Arguments
/// 
/// * `f`: f :: T -> bool, function to filter item
/// * `it`: Iterator to be filtered
pub fn filter<T>(f: impl Fn(&T) -> bool, it: impl Iterator<Item=T>) -> impl Iterator<Item=T> {
    it.filter(f)
}

/// Curry macro of [`filter`]
/// 
/// **Signature**: filter :: (T -> bool) -> Iterator T -> Iterator T
#[macro_export]
macro_rules! filter {
    ($f:expr) => {
        move |it| filter($f, it)
    };
}

/// Used for reverse filter Iterator<T>
/// 
/// # Arguments
/// 
/// * `f`: f :: T -> bool, function to reverse filter item
/// * `it`: Iterator to be filtered
pub fn filter_not<T>(f: impl Fn(&T) -> bool, it: impl Iterator<Item=T>) -> impl Iterator<Item=T> {
    it.filter(move |x| !f(x))
}

/// Curry macro of [`filter_not`]
/// 
/// **Signature**: filter_not :: (T -> bool) -> Iterator T -> Iterator T
#[macro_export]
macro_rules! filter_not {
    ($f:expr) => {
        move |it| filter_not($f, it)
    };
}