
extern crate auto_vec;
use auto_vec::auto_vec;

#[auto_vec]
pub fn foo(arg1: i64, arg2: i32) -> f64 {
    return (arg1 + arg2 as i64) as f64
}

#[auto_vec]
pub fn foo2(arg1: i64, arg2: i32, arg3: u32) -> f64 {
    return (arg1 + arg2 as i64 + arg3 as i64) as f64
}

#[test]
fn test_equ_size_vectors() {
    let mut vec1 = Vec::new();
    vec1.push(1i64);
    vec1.push(2i64);

    let mut vec2 = Vec::new();
    vec2.push(100i32);
    vec2.push(200i32);

    
    let res = foo(vec1, vec2);
    assert_eq!(res[0], 101f64);
    assert_eq!(res[1], 202f64);
}

// Test empty vectors
#[test]
fn test_empty_vectors() {
    let a1: Vec<i64> = Vec::new();
    let a2: Vec<i32> = Vec::new();
    
    let res = foo(a1, a2);
    assert_eq!(res.len(), 0);
}

// Test non-equ-size vectors
#[test]
fn test_non_equ_vectors() {
    let mut a1: Vec<i64> = Vec::new();
    let a2: Vec<i32> = Vec::new();
    
    a1.push(1i64);
    // Should panic in foo
    let _res = foo(a1, a2);
}

// Test three-vectors
#[test]
fn test_three_vectors() {
    let mut vec1 = Vec::new();
    vec1.push(1i64);
    vec1.push(2i64);

    let mut vec2 = Vec::new();
    vec2.push(100i32);
    vec2.push(200i32);

    let mut vec3 = Vec::new();
    vec3.push(100u32);
    vec3.push(200u32);

    let res = foo2(vec1, vec2, vec3);
    assert_eq!(res[0], 201f64);
    assert_eq!(res[1], 402f64);
}

