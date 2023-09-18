use savvy::{
    savvy, sexp::na::NotAvailableValue, IntegerSxp, OwnedIntegerSxp, OwnedRealSxp, OwnedStringSxp,
    StringSxp,
};

/// @export
#[savvy]
fn identity_int1(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedIntegerSxp::new(x.len());

    for (i, &v) in x.iter().enumerate() {
        out[i] = v;
    }

    Ok(out.into())
}

/// @export
#[savvy]
fn sum_int(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut res = 0;
    let mut out = OwnedIntegerSxp::new(1);

    for &v in x.iter() {
        if v.is_na() {
            res = <i32>::na();
            break;
        }
        res += v;
    }

    out[0] = res;

    Ok(out.into())
}

/// @export
#[savvy]
fn to_upper(x: StringSxp) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedStringSxp::new(x.len());

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_elt(i, <&str>::na());
            continue;
        }

        let e_upper = e.to_uppercase();
        out.set_elt(i, e_upper.as_str());
    }

    Ok(out.into())
}
