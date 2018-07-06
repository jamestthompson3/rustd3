
pub fn max(arr: &Vec<f64>) -> f64 {
    *arr.iter().next()
    .map(|mut max| {
        for i in arr {
            if i > max {
                max = i;
            }
        }
        max
    }).unwrap()
}

pub fn min(arr: &Vec<f64>) -> f64 {
    *arr.iter().next()
    .map(|mut min| {
        for i in arr {
            if i < min {
                min = i;
            }
        }
        min
    }).unwrap()
}

pub fn extent(arr: &Vec<f64>) -> Vec<f64> {
    let min = min(arr);
    let max = max(arr);
    return vec![min, max];
}

fn sum_array(val_array:Vec<f64>)-> f64{
    return val_array.iter().sum();
}

fn deviation(arr:&Vec<f64>) -> f64 {
    variance(arr).sqrt()
}

fn mean(val_array:&Vec<f64>) -> f64{
    sum_array(val_array.to_vec()) / val_array.len() as f64
}

pub fn variance(arr: &Vec<f64>) -> f64 {
    if arr.len() < 2 {
            0.0
        } else {
        let mean = mean(arr);
        let num: f64 = arr.iter()
        .map(|x| {
            x - mean
        }).sum();
        let denom = (&arr.len() - 1) as f64;
    (num * num)/denom
    }
}

fn main() {
    let array = vec![1.44,2.23,3.65,5.65,7.66,7.885,4.345,234.33,2.32,556.76,87.69,0.76,45.34,45.56,43.44,45.566,67.86,342.34,344.33,43.45];
    let min = min(&array);
    let max = max(&array);
    let extent = extent(&array);
    let variance = variance(&array);
    let deviation = deviation(&array);
    let mean = mean(&array);
    // println!("extent is {}\n", extent);
    println!("mean is {}\nvariance is {}\ndeviation is {}\nmax is {}\nmin is {}\nextent is {:?}", mean, variance, deviation, max, min, extent);
}
