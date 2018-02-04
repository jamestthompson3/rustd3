
pub fn max(arr: Vec<i32>) -> i32 {
    let arr_iter = arr.iter();
    arr.iter().next()
    .map(|mut max| {
        for i in arr_iter {
            if i > max {
                max = i;
            }
        }
        *max
    }).unwrap()
}

pub fn min(arr: Vec<i32>) -> i32 {
    let mut arr_iter = arr.iter();
    *arr_iter.next()
    .map(|mut min| {
        for i in arr_iter {
            if i < min {
                min = i;
            }
        }
        // print!("{}\n", min);
        min
    }).unwrap()
}

#[no_mangle]
pub fn extent(arr: Vec<i32>) -> Vec<i32> {
    let refer = arr.clone();
    let min = min(arr);
    let max = max(refer);
    return vec![min, max];
}

fn sum_array(val_array:Vec<f64>)-> f64{
    return val_array.iter().sum();
}

fn deviation(arr:Vec<f64>) -> f64 {
    variance(arr).sqrt()
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

fn main() {
    let array = vec![1.44,2.23,3.65,5.65,7.66,7.885,4.345,234.33,2.32,556.76,87.69,0.76,45.34,45.56,43.44,45.566,67.86,342.34,344.33,43.45];
    // let min = min(array);
    // let variance = variance(array);
    let deviation = deviation(array);
    // println!("min is {}\n", min);
    println!("deviation is {}\n", deviation);
}
