use super::models::{Array, ArrayBuilder};

#[test]
fn size_of() {
    assert_eq!(std::mem::size_of::<u64>(), 8);
    assert_eq!(std::mem::size_of::<Option<u64>>(), 16);
}

fn eval_binary<I: Array, O: Array>(arr1: I, arr2: I) -> O {
    if arr1.len() != arr2.len() {
        panic!("array lenghts are not equal")
    }
    let mut builder = O::Builder::with_capacity(arr1.len());
    for (val1, val2) in arr1.iter().zip(arr2.iter()) {
        // builder.push(sql_func(val1, val2));
    }
    builder.finish()
}

#[test]
fn test_eval() {}
