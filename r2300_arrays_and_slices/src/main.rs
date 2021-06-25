use std::{array::FixedSizeArray, mem};

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice nas {} elements", slice.len());
}
fn main() {
    let xs: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let ys: [i32; 60] = [3; 60];
    println!("first element of array: {}", xs[0]);
    println!("second element: {}", xs[1]);

    println!("number of elements in array: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[3..34]);
    //println!("{}", xs[7]); - error within compile, index out of bounds
}
