/// d3 Array functions
/// min, max, extent, sum,
/// mean, median, quantile, variance,
/// deviation

use std::cmp;

fn min(array: []) {
    return cmp::min(array);
}

fn max(array: []) {
    return cmp::max_val(array);
}
