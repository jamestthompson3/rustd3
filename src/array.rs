/// # Array
/// Implements d3 array functions ()[https://github.com/d3/d3-array]

use std::ops::Div;

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

pub fn extent<'a, T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
    let min = min(arr);
    let max = max(arr);
    return vec![min, max];
}

pub fn sum_array<'a, T: PartialOrd + Copy>(arr: &'static [T]) -> T
    where T: std::iter::Sum<&'a T>
{
    return arr.iter().sum();
}

pub fn deviation<T: PartialOrd + Copy>(arr: &[T]) -> f64 {
    variance(arr).sqrt()
}

pub fn mean<T: PartialOrd + Copy + Div>(arr: &[T]) -> f64
{
    let length = arr.len() as f64;
    sum_array(&arr) as f64 / length
}

fn median<T: PartialOrd + Copy>(arr: &[T]) -> f64 {
    1.22
}

pub fn variance<T: PartialOrd + Copy>(arr: &[T]) -> f64 {
    if arr.len() < 2 {
            0.0
        } else {
        let num: T = arr.iter()
        .map(|x| {
            x - mean
        let denom = (&arr.len() - 1) as f64;
    (num * num)/denom
    }
}
}
