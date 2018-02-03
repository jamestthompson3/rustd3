
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

pub fn extent(arr: Vec<i32>) -> Vec<i32> {
    let refer = arr.clone();
    let min = min(arr);
    let max = max(refer);
    return vec![min, max];
}
fn main() {
    let array = vec![1,2,3,4,5,33,44,55,0,123,344,12,3,2,12,33,44,3,1,2,3,12,-23,12,232,34,87,786,786,786,67,78,67,84,5,6];
    // let min = min(array);
    let extent = extent(array);
    // println!("min is {}\n", min);
    println!("extent min is {}\n", extent[0]);
    println!("extent max is {}\n", extent[1]);
}
