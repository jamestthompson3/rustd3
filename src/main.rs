
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
// Copy pasta code
fn max_by_for<I, F, B>(mut it: I, mut f: F) -> Option<I::Item>
     where I: Iterator, F: FnMut(&I::Item) -> B, B: Ord
{
    it.next()
      .map(|mut max| {
        let mut max_val = f(&max);
        for y in it {
            let y_val = f(&y);
            if y_val >= max_val {
                max = y;
                max_val = y_val;
            }
        }
        max

      })
}
pub fn max_index_iter_for(array: &[i32]) -> usize {
    max_by_for(array.iter().enumerate(), |&(_, item)| item).unwrap().0
}

fn main() {
    let array = vec![1,2,3,4,5,33,44,55,0,123,344,12,3,2,12,33,44,3,1,2,3,12,-23,12,232,34,87,786,786,786,67,78,67,84,5,6];
    // let min = min(array);
    let max = max(array);
    // println!("min is {}\n", min);
    println!("max is {}\n", max);
}
