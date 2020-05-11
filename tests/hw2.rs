extern crate auto_vec;
use auto_vec::auto_vec;

#[derive(Copy, Clone)]
pub struct Num {
    x : i64,
}

impl Into<i64> for Num {
   fn into(self) -> i64 {
       self.x
   }
}

#[auto_vec]
fn yay<T: Into<i64>>(a: i32, b: i64, c: T) -> u64 {
     let aa = a as u64;
     let bb : u64 = b as u64;
     let tmp : i64 = c.into();
     let cc = tmp as u64;

     return aa + bb + cc
}

// Test function template
#[test]
fn test_template() {
    let mut vec1 = Vec::new();
    vec1.push(1i32);
    vec1.push(2i32);

    let mut vec2 = Vec::new();
    vec2.push(100i64);
    vec2.push(200i64);


    let mut vec3 = Vec::new();
    let n1 = Num {
        x : 100i64
    };

    let n2 = Num {
        x : 200i64
    };
    vec3.push(n1);
    vec3.push(n2);

    let res = yay(vec1, vec2, vec3);
    assert_eq!(res[0], 201u64);
    assert_eq!(res[1], 402u64);
}
