
#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_error() directly.
            Rf_error("%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}


SEXP savvy_identity_int1_wrapper(SEXP x) {
    SEXP res = savvy_identity_int1(x);
    return handle_result(res);
}

SEXP savvy_sum_int_wrapper(SEXP x) {
    SEXP res = savvy_sum_int(x);
    return handle_result(res);
}

SEXP savvy_to_upper_wrapper(SEXP x) {
    SEXP res = savvy_to_upper(x);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"savvy_identity_int1", (DL_FUNC) &savvy_identity_int1_wrapper, 1},
    {"savvy_sum_int", (DL_FUNC) &savvy_sum_int_wrapper, 1},
    {"savvy_to_upper", (DL_FUNC) &savvy_to_upper_wrapper, 1},

    {NULL, NULL, 0}
};

void R_init_savvyPkg(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}