#' @useDynLib savvyPkg, .registration = TRUE
#' @keywords internal
NULL

#' @export
int_input <- function(x) {
  .Call(int_input__impl, x)
}

#' @export
int_output <- function(len) {
  .Call(int_output__impl, len)
}

#' @export
str_input <- function(x) {
  .Call(str_input__impl, x)
}

#' @export
str_output <- function(len) {
  .Call(str_output__impl, len)
}


