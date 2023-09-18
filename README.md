
<!-- README.md is generated from README.Rmd. Please edit that file -->

# Benchmark savvy vs extendr

<!-- badges: start -->
<!-- badges: end -->

## Identity

``` r
bench_identity <- function(x) {
  x <- force(x)
  results = bench::mark(
    extendr1 = extendrPkg::identity_int1(x),
    extendr2 = extendrPkg::identity_int2(x),
    savvy    = savvyPkg::identity_int1(x)
  )
  
  ggplot2::autoplot(results)
}

bench_identity(1L)
#> Loading required namespace: tidyr
```

![](README_files/figure-gfm/unnamed-chunk-2-1.png)<!-- -->

``` r
bench_identity(1L:1000L)
```

![](README_files/figure-gfm/unnamed-chunk-2-2.png)<!-- -->

## Sum

``` r
bench_sum <- function(x) {
  x <- force(x)
  results = bench::mark(
    extendr = extendrPkg::sum_int(x),
    savvy   = savvyPkg::sum_int(x)
  )
  
  ggplot2::autoplot(results)
}

bench_sum(1L)
```

![](README_files/figure-gfm/unnamed-chunk-3-1.png)<!-- -->

``` r
bench_sum(1L:10000L) # ALTREP
```

![](README_files/figure-gfm/unnamed-chunk-3-2.png)<!-- -->

``` r
bench_sum(rep(1L, 10000)) # non-ALTREP
```

![](README_files/figure-gfm/unnamed-chunk-3-3.png)<!-- -->

## String conversion

``` r
bench_to_upper <- function(x) {
  x <- force(x)
  results = bench::mark(
    extendr = extendrPkg::to_upper(x),
    savvy   = savvyPkg::to_upper(x)
  )
  
  ggplot2::autoplot(results)
}

bench_to_upper("a")
```

![](README_files/figure-gfm/unnamed-chunk-4-1.png)<!-- -->

``` r
bench_to_upper(letters)
```

![](README_files/figure-gfm/unnamed-chunk-4-2.png)<!-- -->

``` r
bench_to_upper(rep("a", 10000))
```

![](README_files/figure-gfm/unnamed-chunk-4-3.png)<!-- -->
