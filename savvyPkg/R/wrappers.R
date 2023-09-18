#' @useDynLib savvyPkg, .registration = TRUE
#' @keywords internal
NULL

#' @export
identity_int1 <- function(x) {
  .Call(savvy_identity_int1, x)
}

#' @export
sum_int <- function(x) {
  .Call(savvy_sum_int, x)
}

#' @export
to_upper <- function(x) {
  .Call(savvy_to_upper, x)
}


