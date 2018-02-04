/// d3 Array functions
/// min, max, extent, sum,
/// mean, median, quantile, variance,
/// deviation

// variance
// there are two ways of calculating the variance we could profile them
//  - 1. direct as central 2nd order moment (https://en.wikipedia.org/wiki/Moment_(mathematics))divided by the length of the vector
//   - 2. "mean of square minus square of mean" (see https://en.wikipedia.org/wiki/Variance)

pub fn min(arr: Vec<i32>) -> i32 {
    let mut arr_iter = arr.iter();
    *arr_iter.next()
    .map(|mut min| {
        for i in arr_iter {
            if i < min {
                min = i;
            }
        }
        min
    }).unwrap()
}
pub fn max(arr: Vec<i32>) -> i32 {
    let mut arr_iter = arr.iter();
    *arr_iter.next()
    .map(|mut max| {
        for i in arr_iter {
            if i > max {
                max = i;
            }
        }
        max
    }).unwrap()
}

fn sum_array(val_array:Vec<f64>)-> f64{
    return val_array.iter().sum();
}

fn mean(val_array:Vec<f64>) -> f64{
    let arr = val_array.clone();
    sum_array(val_array) / arr.len() as f64
}

pub fn variance(arr: Vec<f64>) -> f64 {
    let referrence = arr.clone();
    if arr.len() < 2 {
            0.0
        } else {
        let mean = mean(referrence);
        let num: f64 = arr.iter()
        .map(|x| {
            x - mean
        }).sum();
        let denom = (&arr.len() - 1) as f64;
    (num * num)/denom
    }
}

pub fn extent(arr: Vec<i32>) -> Vec<i32> {
    let refer = arr.clone();
    let min = min(arr);
    let max = max(refer);
    return vec![min, max];
}
