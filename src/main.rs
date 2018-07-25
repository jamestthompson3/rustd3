extern crate d3_calcs;

use d3_calcs::array::{deviation, extent, max, mean, min, variance};

fn main() {
    let array = vec![
        1.44, 2.23, 3.65, 5.65, 7.66, 7.885, 4.345, 234.33, 2.32, 556.76, 87.69, 0.76, 45.34,
        45.56, 43.44, 45.566, 67.86, 342.34, 344.33, 43.45,
    ];
    let min = min(&array);
    let max = max(&array);
    let extent = extent(&array);
    let variance = variance(&array);
    let deviation = deviation(&array);
    let mean = mean(&array);
    println!(
        "mean is {}\nvariance is {}\ndeviation is {}\nmax is {}\nmin is {}\nextent is {:?}",
        mean, variance, deviation, max, min, extent
    );
}
