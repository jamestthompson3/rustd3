#![feature(extern_prelude)]
/// # Array
/// Implements d3 array functions ()[https://github.com/d3/d3-array]

extern crate num_traits;
use std::ops::Div;
use std::ops::Add;
use std::convert::*;
use std::ops::Mul;

pub trait MyInt: Copy + Into<f64> {
        fn blank();

}

impl MyInt for i32 {
    fn blank() {
        unimplemented!()
    }
}

impl MyInt for f32 {
    fn blank() {
        unimplemented!()
    }
}

pub fn mean<T>(arr: &[T]) -> f64
    where T: MyInt
{
    let sum_array = arr.iter().fold(0f64 ,|a, &b| a + b.into());
    let length = arr.len() as f64;
    sum_array / length
}

// pub fn sum_array<'a, T: PartialOrd + Copy>(arr: &'static [T]) -> T
//     where T: std::iter::Sum<&'a T>
// {
//     return arr.iter().sum();
// }



// pub fn mean<T: PartialOrd + Copy + Div>(arr: &[T]) -> T
// {
//     let length = arr.len() as f64;
//     sum_array(&arr) as f64 / length
// }

pub fn max<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut max = arr[0];
        for &i in arr.iter() {
            if i < max {
                max = i;
            }
        }
        max
 }

pub fn min<T: PartialOrd + Copy>(arr: &[T]) -> T {
       let mut max = arr[0];
        for &i in arr.iter() {
            if i < max {
                max = i;
            }
        }
        max
}

// pub fn extent<'a, T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
//     let min = min(arr);
//     let max = max(arr);
//     return vec![min, max];
// }

// pub fn deviation<T: PartialOrd + Copy>(arr: &[T]) -> f64 {
//     variance(arr).sqrt()
// }

// fn median<T: PartialOrd + Copy>(arr: &[T]) -> f64 {
//     1.22
// }

// pub fn variance<T: PartialOrd + Copy>(arr: &[T]) -> f64 {
//     if arr.len() < 2 {
//             0.0
//         } else {
//         let num: T = arr.iter()
//         .map(|x| {
//             x - mean
//         let denom = (&arr.len() - 1) as f64;
//     (num * num)/denom
//     }
// }
// }
