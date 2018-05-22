[![Build Status](https://travis-ci.org/fgoinai/KaguyaRs.svg?branch=master)](https://travis-ci.org/fgoinai/KaguyaRs)

# Category
1. Introduction
2. Usage
3. Requirement
4. Basic function
5. Typeclass in rust as trait
6. Typeclass impl for std struct and enum of Rust
7. Contribution and developing
8. Thanks List
9. LICENSE

### Introduction
KaguyaRs is a **TOY** functional library of Rust.

In fact, this is just the way I get familiar with Rust.
I will try my best to implement as many as utilities.
But NEVER try this in production environment UNTIL:

1. Someday, I remove `TOY` in Introduction. OR
2. I publish 1.0-stable version.

Key concerns of this library:

1. May all the functions in this library are pure IF I CAN.
2. Similar as 1, but pure -> lazy
3. Base on 1, heavy clone may occur, persistance data structure will be introduced later to achive 4
4. Performance. If not necessary `trait object` will be avoided.
5. Implementation may be ugly and buggy, just feel free to issue it for any ideas.

### Usage

### Requirement

### Basic function
Basic function and its signature provided is listed below, function based on map will be applied later

There are three versions: curry or not, macro

For curry version, each param will form a new closure, for example
```rust
let f = map(|x| x+1, [1,2,3,4,5]);
assert_eq!(f, [2,3,4,5,6]);

let f2 = c_map(|x| x+1)([1,2,3,4,5]); // (Number -> Number) -> [Number] -> [Number]
assert_eq!(f2, [2,3,4,5,6])
```
For macro version, please refer test for further details as macro can curry automatically depends on the way to invoke macro.

All function with more than one param will have a curry version start with "c_", like c_map

For details, please refer TODO.md.

### Typeclass in rust as trait
In Haskell, there are so many useful typeclass such as Monad, Functor, Applicative etc. I will implement as much as typeclass as I can. This can greatly empower ability of Functional Programming in Rust.

### Typeclass impl for std struct and enum of Rust
There are some struct and enum that are widely used in Rust, e.g. Vec, HashMap, Option, Result etc. They can be treat as ADT that can impl typeclass of Haskell like Monad. I will implement them to enhance the power of std struct and enum.

### Contribution and developing
Feel free to contribute and develop base on this lib. Make sure you MUST follow the constraint of LICENSE.

If you are interested to develop this library together, just make PRs with new issues **OR** send me a email to explain what you want to do for this library as a contributor.

For PR and any other questions, please create a new issue for following.

### Thanks List
- [RamdaJS](https://ramdajs.com/) - inspiration of most functions
- [fluxxu@zhihu](https://www.zhihu.com/people/fluxxu/activities) - pipe macro

### LICENSE
Apache License v2.