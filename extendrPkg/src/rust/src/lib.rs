use extendr_api::prelude::*;

/// @export
#[extendr]
fn identity_int1(x: Integers) -> Integers {
    x
}

/// @export
#[extendr]
fn identity_int2(x: Vec<i32>) -> Vec<i32> {
    x
}

/// @export
#[extendr]
fn sum_int(x: Integers) -> Rint {
    <Rint as std::iter::Sum>::sum(x.iter())
}

/// @export
#[extendr]
fn to_upper(x: Strings) -> Strings {
    x.iter().map(|v| v.to_uppercase()).collect()
}


extendr_module! {
    mod extendrPkg;
    fn identity_int1;
    fn identity_int2;
    fn sum_int;
    fn to_upper;
}
