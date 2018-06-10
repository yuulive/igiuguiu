// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(trace_macros)]

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

#[test]
// fn map and curry map
fn map() {
    use kaguya_rs::basic_fn::fun::map;
    let v = vec![1,2,3];
    let result: Vec<i32> = map(|x| x+1, v.iter()).collect();
    assert_eq!(result, vec![2,3,4]);

    let curry = map!(|x| x+1);
    let result2: Vec<i32> = curry(v.iter()).collect();
    assert_eq!(result2, vec![2,3,4]);
}

#[test]
// fn filter and curry filter
fn filter() {
    use kaguya_rs::basic_fn::fun::filter;
    let v = vec![1,2,3];
    let odd = filter(|&x| x & 1 == 1, v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(odd, vec![1,3]);

    let curry = filter!(|&x| x&1 == 0);
    let even = curry(v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(even, vec![2]);
}

#[test]
// fn filter_not and curry filter_not
fn filter_not() {
    use kaguya_rs::basic_fn::fun::filter_not;
    let v = vec![1,2,3];
    let even = filter_not(|&x| x & 1 == 1, v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(even, vec![2]);

    let curry = filter_not!(|&x| x&1 == 0);
    let odd = curry(v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(odd, vec![1,3]);
}

#[test]
// fn foldl and curry foldl
fn foldl() {
    use kaguya_rs::basic_fn::fun::foldl;
    let v = vec![1,2,3];
    let result = foldl(4, |x,y| x*y, v.iter());
    assert_eq!(result, 24);

    let curry1 = foldl!(5);
    let c_result1 = curry1(|x,y| x*y, v.iter());
    assert_eq!(c_result1, 30);

    let curry2 = foldl!(6, |x,y| x-y);
    let c_result2 = curry2(v.iter());
    assert_eq!(c_result2, 0);

    let step_curry = foldl!(0=>);
    let step_curry_2 = step_curry(|x,y| x+y);
    assert_eq!(step_curry_2(v.iter()), 6);
}

#[test]
// fn foldr and curry foldr
fn foldr() {
    use kaguya_rs::basic_fn::fun::foldr;
    let v = vec!["Houraisan","Kaguya"];
    let result = foldr("".to_string(), |x,y| x + "<|>" + y, v.iter());
    assert_eq!(result, "<|>Kaguya<|>Houraisan");

    let curry1 = foldr!("This is:".to_string());
    let c_result1 = curry1(|x,&y| x+" "+y, v.iter());
    assert_eq!(c_result1, "This is: Kaguya Houraisan");
    
    let curry2 = foldr!("すごい！".to_string(), |x,&y| x+" "+y);
    let c_result2 = curry2(v.iter());
    assert_eq!(c_result2, "すごい！ Kaguya Houraisan");

    let step_curry = foldr!("楽しい〜".to_string()=>);
    let step_curry_2 = step_curry(|x,&y| x+" "+y);
    assert_eq!(step_curry_2(v.iter()), "楽しい〜 Kaguya Houraisan");
}

#[test]
// fn sum and macro sum
fn sum() {
    use kaguya_rs::basic_fn::fun::sum;
    let result = 10;
    assert_eq!(sum(1..=4), result);
    assert_eq!(sum!(1;4), result);
    assert_eq!(sum!(1,2,3,4), result);
}

#[test]
// macro ls: list comprehension
fn ls() {
    // 1. function on item;iter=>function to check condition
    assert_eq!(ls![|x| x+1; 1..=5 => |x| x&1==0], vec![3,5]);
    // 2. iter=>function to check condition
    assert_eq!(ls![0..=4 => |x| x&1==0], vec![0,2,4]);
    // 3. function on item;iter
    assert_eq!(ls![|x| x*x; 0..=4], vec![0,1,4,9,16]);
    // 4. iter
    assert_eq!(ls![0..=4], vec![0,1,2,3,4]);
}

#[test]
// fn head
fn head() {
    use kaguya_rs::basic_fn::fun::head;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(None, head(empty_vec.iter()));
    assert_eq!(Some(&1), head(vec.iter()));
}

#[test]
// fn tail
fn tail() {
    use kaguya_rs::basic_fn::fun::tail;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1, 2, 3];

    assert_eq!(None, tail(empty_vec.iter()));
    assert_eq!(Some(vec![&2, &3]), tail(vec.iter()));
}

#[test]
// fn last
fn last() {
    use kaguya_rs::basic_fn::fun::last;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1, 2, 3];

    assert_eq!(None, last(empty_vec.iter()));
    assert_eq!(Some(&3), last(vec.iter()));
}

#[test]
// fn init
fn init() {
    use kaguya_rs::basic_fn::fun::init;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(None, init(empty_vec.iter()));
    assert_eq!(Some(vec![&1,&2]), init(vec.iter()));
}

#[test]
// fn and macro skip
fn skip() {
    use kaguya_rs::basic_fn::fun::skip;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(Vec::new() as Vec<&i8>, skip(1,empty_vec.iter()));
    assert_eq!(vec![&2,&3], skip(1, vec.iter()));

    let curry = skip!(1);
    assert_eq!((Vec::new() as Vec<&i8>), curry(empty_vec.iter()));
    assert_eq!(vec![&2, &3], curry(vec.iter()));
}

#[test]
// fn and macro take
fn take() {
    use kaguya_rs::basic_fn::fun::take;
    let empty_rec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(Vec::new() as Vec<&i8>, take(2, empty_rec.iter()));
    assert_eq!(vec![&1,&2], take(2, vec.iter()));

    let curry = take!(2);
    assert_eq!(Vec::new() as Vec<&i8>, curry(empty_rec.iter()));
    assert_eq!(vec![&1,&2], curry(vec.iter()));
}

#[test]
// fn and macro product
fn product() {
    use kaguya_rs::basic_fn::fun::product;
    assert_eq!(product(1..=5), 120);

    assert_eq!(product!(1;5), 120);
    assert_eq!(product!(1,2,3,4,5), 120);
}

#[test]
// fn length
fn length() {
    use kaguya_rs::basic_fn::fun::length;
    assert_eq!(length(vec![1,2,3,4,5].iter()), 5);
    assert_eq!(length(0..0), 0);
}

#[test]
// fn reverse
fn reverse() {
    use kaguya_rs::basic_fn::fun::reverse;
    assert_eq!(reverse(1..=5), vec![5,4,3,2,1]);
}

#[test]
// fn and macro concat
fn concat() {
    use kaguya_rs::basic_fn::fun::concat;
    assert_eq!(concat(0..1,1..2), vec![0,1]);
    assert_eq!(
        concat!(0..1;1..2;2..3;3..=4;vec![5,6].iter()),
        vec![0,1,2,3,4,5,6]
    );
}

#[test]
// fun id
fn id() {
    use kaguya_rs::basic_fn::fun::id;
    let vec = vec![1,2,3];
    assert_eq!(id(vec.clone()), vec);
}

#[test]
// macro fst
fn fst() {
    assert_eq!(fst!(("a", 1)), "a");
    let f = fst!(>i8,f32,char);
    assert_eq!(f((1,1.2,'3')), 1);
    let a = (1,1.2,'3');
    assert_eq!(fst!(a), 1);
}

#[test]
// macro snd
fn snd() {
    assert_eq!(snd!((1.1, "Bravo")), "Bravo");
    let f = snd!(>String,String,String);
    assert_eq!(f(("Houraisan".to_string(), "Kaguya".to_string(), "美しい".to_string())), "Kaguya");
    let a = (1,3,2.5);
    assert_eq!(snd!(a), 3);
}

#[test]
// fn neg
fn neg() {
    use kaguya_rs::basic_fn::fun::neg;
    assert_eq!(neg(1), -1);
    assert_eq!(neg(-1), 1);
}

#[test]
// macro abs
fn abs() {
    assert_eq!(abs!(-1_i32), 1);
    assert_eq!(abs!(1_i32), 1);

    let f = abs!(>i32);
    assert_eq!(f(-1), 1);
    assert_eq!(f(1), 1);
}

#[test]
// macro signum
fn signum() {
    assert_eq!(signum!(-100_i32), -1);
    assert_eq!(signum!(0_i32), 0);
    assert_eq!(signum!(5_i32), 1);

    let f = signum!(>i32);
    assert_eq!(f(-100), -1);
    assert_eq!(f(0), 0);
    assert_eq!(f(5), 1);
}

#[test]
// fn and macro rem
fn rem() {
    use kaguya_rs::basic_fn::fun::rem;
    assert_eq!(rem(3, 2), 1);
    assert_eq!(rem!(3, 2), 1);
    let f = rem!(3_i32);
    assert_eq!(f(2), 1);
}

#[test]
// macro odd/even
fn odd_and_even() {
    assert!(even!(2));
    assert!(odd!(1));

    let f1 = even!(>i32);
    let f2 = odd!(>i32);
    assert!(f1(2));
    assert!(f2(1));
}

#[test]
// macro always
fn always() {
    assert_eq!(always!(1, 'a'), 1);
    assert_eq!(always!(1)('a'), 1);

    let f = always!();
    assert_eq!(f(1, 'a'), 1);

    let f2 = always!(=>);
    assert_eq!(f2(1)('a'), 1);
}

#[test]
// fn and macro add/sub/mul/div
fn add_sub_mul_div() {
    use kaguya_rs::basic_fn::fun::{add, sub, mul, div};
    // add
    assert_eq!(add(1,2), 1 + 2);
    assert_eq!(add!(1, 2), 1+2);
    assert_eq!(add!(1)(2), 1+2);

    // sub
    assert_eq!(sub(9,1),9-1);
    assert_eq!(sub!(9,1),9-1);
    assert_eq!(sub!(9)(1), 9-1);

    //mul
    assert_eq!(mul(2,3),2*3);
    assert_eq!(mul!(2,3),2*3);
    assert_eq!(mul!(2)(3),2*3);

    // div
    assert_eq!(div(10,5), 10/5);
    assert_eq!(div!(10,5), 10/5);
    assert_eq!(div!(10)(5), 10/5);
}

#[test]
// fn and macro find
fn find() {
    use std::collections::HashMap;
    use kaguya_rs::basic_fn::fun::find;
    let mut m = HashMap::new();
    m.insert('a', 1);
    m.insert('b', 2);
    m.insert('c', 3);
    assert_eq!(find(&'a', m.iter()), Some((&'a', &1)));
    let f = find!(&'b');
    assert_eq!(f(m.iter()), Some((&'b', &2)));
    assert_eq!(find!(&'d', m.iter()), None);
}

#[test]
// fn sorted
fn sorted() {
    use kaguya_rs::basic_fn::fun::sorted;
    let vec = vec![1,5,3,7,8,3,9,3,2];
    assert_eq!(sorted(vec.iter()).map(|x| *x).collect::<Vec<i32>>(), vec![1,2,3,3,3,5,7,8,9]);
}

#[test]
// fn and macro sorted_by
fn sorted_by() {
    use kaguya_rs::basic_fn::fun::sorted_by;
    use std::cmp::Ordering;
    let vec = vec![1,5,3,4,2];
    assert_eq!(vec![5,4,3,2,1], sorted_by(|x,y| y.cmp(x), vec.iter()).map(|x| *x).collect::<Vec<i32>>());

    fn cmp<'a, 'b, T: Ord>(x: &'a T, y: &'b T) -> Ordering {
        x.cmp(y)
    }
    let f = sorted_by!(cmp);
    assert_eq!(vec![1,2,3,4,5], f(vec.iter()).map(|x| *x).collect::<Vec<i32>>());
}

// #[test]
// // macro sorted_with
// fn sorted_with() {
//     let ls = vec![(1,'a'), (1,'b'), (2,'a')];
//     trace_macros!(true);
//     let f = sorted_with!(
//         |x: (&i32, &char),y: (&i32, &char)| x.0.cmp(y.0),
//         |x: (&i32, &char),y: (&i32, &char)| y.1.cmp(x.1)
//     );
//     assert_eq!(vec![(1,'b'), (1,'a'), (2,'a')], f(ls.iter()));
//     trace_macros!(false);
// }

#[test]
// fn and macro zip
fn zip() {
    use kaguya_rs::basic_fn::fun::zip;
    let ls1 = vec![1,2,3];
    let ls2 = vec!['a','b','c'];
    assert_eq!(zip(ls1.iter(), ls2.iter()).map(|(x,y)| (*x,*y)).collect::<Vec<_>>(), vec![(1,'a'), (2,'b'), (3,'c')]);

    let it = ls1.iter();
    let mac = zip!(it);
    assert_eq!(mac(ls2.iter()).map(|(x,y)| (*x,*y)).collect::<Vec<_>>(), vec![(1,'a'), (2,'b'), (3,'c')]);
}

#[test]
// fn and macro zip_with
fn zip_with() {
    use kaguya_rs::basic_fn::fun::zip_with;
    let ls1 = vec![1,2,3];
    let ls2 = vec![1,0,1];
    assert_eq!(zip_with(move |(x,y)| x&y == 0, ls1.iter(), ls2.iter()).collect::<Vec<_>>(), vec![false, true, false]);

    let mac1 = zip_with!(move |(x,y)| x&y == 0);
    let mac2 = zip_with!(move |(x,y)| {x&y == 0} =>);
    let it = ls1.iter();
    let mac3 = zip_with!(move |(x,y)| x&y == 0, it);
    assert_eq!(mac1(ls1.iter(), ls2.iter()).collect::<Vec<_>>(), vec![false, true, false]);
    assert_eq!(mac2(ls1.iter())(ls2.iter()).collect::<Vec<_>>(), vec![false, true, false]);
    assert_eq!(mac3(ls2.iter()).collect::<Vec<_>>(), vec![false, true, false]);
}