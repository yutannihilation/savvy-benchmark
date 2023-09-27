use extendr_api::prelude::*;

/// @export
#[extendr(use_try_from = true)]
fn int_input(x: Integers) -> i32 {
    let mut res = 0;

    for i in x.iter() {
        res += i.inner();
    }

    res
}

/// @export
#[extendr(use_try_from = true)]
fn int_input_vec(x: Vec<i32>) -> i32 {
    let mut res = 0;

    for i in x.iter() {
        res += *i;
    }

    res
}

/// @export
#[extendr(use_try_from = true)]
fn int_output(len: i32) -> Integers {
    let mut out = Integers::new(len as _);

    for i in 0..len {
        out.set_elt(i as _, Rint::from(i));
    }

    out
}

/// @export
#[extendr(use_try_from = true)]
fn str_input(x: Strings) -> String {
    let mut res = String::new();

    for e in x.iter() {
        if e.is_na() {
            continue;
        }
        res.push_str(e.as_str());
    }

    res
}

/// @export
#[extendr(use_try_from = true)]
fn str_output(len: i32) -> Strings {
    let mut out = Strings::new(len as _);

    for i in 0..len {
        out.set_elt(i as _, Rstr::from_string(&i.to_string()));
    }

    out
}

extendr_module! {
    mod extendrPkg;
    fn int_input;
    fn int_input_vec;
    fn int_output;
    fn str_input;
    fn str_output;
}
