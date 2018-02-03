use std::cmp;
use std::ptr;

// Min reference: https://github.com/d3/d3-array/blob/master/README.md#min
// Source: https://github.com/d3/d3-array/blob/master/src/min.js
pub fn min(arr: Vec<i32>) -> i32 {
    let mut min = None;
    let arr_iter = arr.into_iter();
    for i in arr_iter {
        if i < arr_iter.next().unwrap(){
            min = Some(i);
        }
    }
    min.unwrap()
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
    let array = vec![1,2,3,4,5,33,44,55,4,5,6,0];
    let min = min(array);
    println!("min is {}", min);
}
