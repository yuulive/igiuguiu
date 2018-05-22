// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate kaguya_rs;

#[test]
/// macro compose!
fn compose() {
    let f = compose! {
        |x| x+1, |x| x*2
    };
    assert_eq!(f(3), 7);
    assert_eq!(f(-1), -1);

    fn add_one(x: i64) -> i64 {
        x + 1
    }
    fn multi_two(x: i64) -> i64 {
        x * 2
    }
    let f2 = compose!(add_one, multi_two);
    
    assert_eq!(f2(3), 7);
    assert_eq!(f2(-1), -1);
}

#[test]
/// macro compose! for type projection
fn compose_type_projection() {
    // f :: &str -> (&str -> Vec String -> usize) -> usize
    let f = compose! {
        |x: Vec<&str>| x.len(),
        |x: &'static str| x.split(' ').collect()
    };
    assert_eq!(f("Houraisan Kaguya"), 2);
}

#[test]
/// macro pipe!
fn pipe() {
    let f = pipe! {
        |x| x+1, |x| x*2
    };
    assert_eq!(f(3), 8);
    assert_eq!(f(-1), 0);

    fn add_one(x: i64) -> i64 {
        x + 1
    }
    fn multi_two(x: i64) -> i64 {
        x * 2
    }
    let f2 = pipe!(add_one, multi_two);
    
    assert_eq!(f2(3), 8);
    assert_eq!(f2(-1), 0);
}

#[test]
/// macro pipe! for type projection
fn pipe_type_projection() {
    // f :: &str -> (&str -> Vec String -> usize) -> usize
    let f = pipe! {
        |x: &'static str| x.split(' ').collect(),
        |x: Vec<&str>| x.len()        
    };
    assert_eq!(f("Houraisan Kaguya"), 2);
}