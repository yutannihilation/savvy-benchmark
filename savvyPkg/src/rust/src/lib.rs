use savvy::{
    savvy, sexp::na::NotAvailableValue, IntegerSxp, OwnedIntegerSxp, OwnedRealSxp, OwnedStringSxp,
    StringSxp,
};

/// @export
#[savvy]
fn int_input(x: IntegerSxp) -> savvy::Result<savvy::SEXP> {
    let mut res = 0;

    for &i in x.iter() {
        res += i;
    }

    let out: savvy::OwnedIntegerSxp = res.try_into()?;
    Ok(out.into())
}

/// @export
#[savvy]
fn int_output(len: i32) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedIntegerSxp::new(len as _)?;

    for i in 0..len {
        out[i as _] = i;
    }

    Ok(out.into())
}

/// @export
#[savvy]
fn str_input(x: StringSxp) -> savvy::Result<savvy::SEXP> {
    let mut res = String::new();

    for e in x.iter() {
        if e.is_na() {
            continue;
        }
        res.push_str(e);
    }

    let out: savvy::OwnedStringSxp = res.try_into()?;
    Ok(out.into())
}

/// @export
#[savvy]
fn str_output(len: i32) -> savvy::Result<savvy::SEXP> {
    let mut out = OwnedStringSxp::new(len as _)?;

    for i in 0..len {
        out.set_elt(i as _, &i.to_string())?;
    }

    Ok(out.into())
}
