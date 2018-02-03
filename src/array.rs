/// d3 Array functions
/// min, max, extent, sum,
/// mean, median, quantile, variance,
/// deviation

// variance
// there are two ways of calculating the variance we could profile them
//  - 1. direct as central 2nd order moment (https://en.wikipedia.org/wiki/Moment_(mathematics))divided by the length of the vector
//   - 2. "mean of square minus square of mean" (see https://en.wikipedia.org/wiki/Variance)



use std::cmp;

fn min(array: []) {
    return cmp::min(array);
}

fn max(array: []) {
    return cmp::max_val(array);
}


fn sum_array(val_array:Vec<f64>)-> f64{
    let result = val_array.iter().fold(0f64,|a, &b| a + b);
    return result
}


fn mean(val_array:Vec<f64>) -> f64{
    let n = val_array.iter().len();
    let sum_n = sum_array(val_array);
    let result = sum_n/n;
    return result;
}


