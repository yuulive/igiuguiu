// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Used for data projection via mapping function.
/// 
/// # Arguments
/// 
/// * `f`: f :: T -> U
/// * `it`: Iterator<T>
pub fn map<T,U>(f: impl Fn(T) -> U, it: impl Iterator<Item=T>) -> impl Iterator<Item=U> {
    it.map(f)
}

/// Curry macro of [`map`]
/// 
/// **Signature**: map :: (T -> U) -> Iterator T -> Iterator U
#[macro_export]
macro_rules! map {
    ($f:expr) => {
        move |it| map($f, it)
    };
}