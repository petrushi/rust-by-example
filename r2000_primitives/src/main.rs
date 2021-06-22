#![allow(unused)]
fn main() {
    let logical: bool = true;
    let a_float: f64 = 3.6;
    let an_integer: i64 = 5323523523555555555;
    let default_integer = 10;
    let default_float = 3.5;
    let mut inferred_type = 12; 
    inferred_type = 42944929423i64; // no error only with int from start
    let mut mutable = 15;
    mutable = 51;
    // mutable = true; - error of course
    let mutable = true; // overwritten with shadowing

}
